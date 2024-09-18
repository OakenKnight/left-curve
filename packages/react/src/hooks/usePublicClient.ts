import {
  type GetPublicClientParameters,
  type GetPublicClientReturnType,
  getPublicClient,
  watchPublicClient,
} from "@leftcurve/connect-kit";

import { useSyncExternalStoreWithSelector } from "use-sync-external-store/shim/with-selector";
import { useConfig } from "./useConfig";

import type { Config, ConfigParameter, Prettify } from "@leftcurve/types";

export type UsePublicClientParameters<config extends Config = Config> = Prettify<
  GetPublicClientParameters & ConfigParameter<config>
>;

export type UsePublicClientReturnType = GetPublicClientReturnType;

export function usePublicClient<config extends Config = Config>(
  parameters: UsePublicClientParameters<config> = {},
): UsePublicClientReturnType {
  const config = useConfig(parameters);

  return useSyncExternalStoreWithSelector(
    (onChange) => watchPublicClient(config, { onChange }),
    () => getPublicClient(config, parameters),
    () => getPublicClient(config, parameters),
    (x) => x,
    (a, b) => a?.uid === b?.uid,
  );
}
