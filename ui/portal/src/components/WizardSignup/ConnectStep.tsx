import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Select, SelectItem, useWizard } from "@dango/shared";
import { useAccount, useConfig, useConnectors, usePublicClient } from "@left-curve/react";
import { useMutation } from "@tanstack/react-query";

import {
  createWebAuthnCredential,
  ethHashMessage,
  secp256k1RecoverPubKey,
} from "@left-curve/crypto";
import { encodeBase64, encodeUtf8 } from "@left-curve/encoding";
import { computeAddress, createAccountSalt, createKeyHash } from "@left-curve/sdk";
import { AccountType, ConnectionStatus, KeyAlgo } from "@left-curve/types";
import { getNavigatorOS, getRootDomain, wait } from "@left-curve/utils";

import { Button } from "@dango/shared";

import type { EIP1193Provider, Key } from "@left-curve/types";
import type { DangoAppConfigResponse } from "@left-curve/types/dango";

export const ConnectStep: React.FC = () => {
  const navigate = useNavigate();

  const config = useConfig();
  const client = usePublicClient();

  const { status } = useAccount();
  const connectors = useConnectors();

  const { data } = useWizard<{ username: string }>();
  const { username } = data;

  const {
    mutateAsync: createAccount,
    isError,
    isPending,
  } = useMutation({
    mutationFn: async (connectorId: string) => {
      try {
        const connector = connectors.find((c) => c.id === connectorId);
        if (!connector) throw new Error("error: missing connector");
        const challenge = "Please sign this message to confirm your identity.";
        const { key, keyHash } = await (async () => {
          if (connectorId === "passkey") {
            const { id, getPublicKey } = await createWebAuthnCredential({
              challenge: encodeUtf8(challenge),
              user: {
                name: `${getNavigatorOS()} ${new Date().toLocaleString()}`,
              },
              rp: {
                name: window.document.title,
                id: getRootDomain(window.location.hostname),
              },
              authenticatorSelection: {
                residentKey: "preferred",
                requireResidentKey: false,
                userVerification: "preferred",
              },
            });

            const publicKey = await getPublicKey();
            const key: Key = { secp256r1: encodeBase64(publicKey) };
            const keyHash = createKeyHash({ credentialId: id, keyAlgo: KeyAlgo.Secp256r1 });

            return { key, keyHash };
          }

          const provider = await (
            connector as unknown as { getProvider: () => Promise<EIP1193Provider> }
          ).getProvider();

          const [controllerAddress] = await provider.request({ method: "eth_requestAccounts" });
          const signature = await provider.request({
            method: "personal_sign",
            params: [challenge, controllerAddress],
          });

          const pubKey = await secp256k1RecoverPubKey(ethHashMessage(challenge), signature, true);

          const key: Key = { secp256k1: encodeBase64(pubKey) };
          const keyHash = createKeyHash({ pubKey, keyAlgo: KeyAlgo.Secp256k1 });

          return { key, keyHash };
        })();

        const { addresses } = await client.getAppConfig<DangoAppConfigResponse>();
        const accountCodeHash = await client.getAccountTypeCodeHash({
          accountType: AccountType.Spot,
        });

        const salt = createAccountSalt({ key, keyHash, username });
        const address = computeAddress({
          deployer: addresses.accountFactory,
          codeHash: accountCodeHash,
          salt,
        });

        const response = await fetch("https://mock-ibc.left-curve.workers.dev", {
          method: "POST",
          body: JSON.stringify({ address }),
        });
        if (!response.ok) throw new Error("error: failed to send funds");

        await client.registerUser({ key, keyHash, username });
        // TODO: Do pooling instead of wait to check account creation
        await wait(1000);
        await connector.connect({ username, chainId: config.chains[0].id });
      } catch (err) {
        console.log(err);
        throw err;
      }
    },
  });

  useEffect(() => {
    if (status !== ConnectionStatus.Connected) return;
    navigate("/");
  }, [navigate, status]);

  return (
    <div className="flex flex-col w-full gap-6">
      <Button fullWidth onClick={() => createAccount("passkey")} isLoading={isPending}>
        Signup with Passkey
      </Button>
      {isError ? (
        <p className="text-typography-rose-600 text-center text-xl">
          We couldn't complete the request
        </p>
      ) : null}
      <Select
        label="login-methods"
        placeholder="Alternative sign up methods"
        isDisabled={isPending}
        position="static"
        onSelectionChange={(connectorId) => createAccount(connectorId.toString())}
      >
        {connectors
          .filter((c) => c.id !== "passkey")
          .map((connector) => {
            return (
              <SelectItem key={connector.id}>
                <div className="flex gap-2">
                  <img
                    src={connector.icon}
                    aria-label="connector-image"
                    className="w-6 h-6 rounded"
                  />
                  <span>{connector.name}</span>
                </div>
              </SelectItem>
            );
          })}
      </Select>
    </div>
  );
};
