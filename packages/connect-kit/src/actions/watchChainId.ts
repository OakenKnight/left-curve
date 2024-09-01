import type { Config } from "@leftcurve/types";
import type { GetChainIdReturnType } from "./getChainId";

export type WatchChainIdParameters<config extends Config = Config> = {
  onChange(chainId: GetChainIdReturnType<config>, prevChainId: GetChainIdReturnType<config>): void;
};

export type WatchChainIdReturnType = () => void;

export function watchChainId<config extends Config>(
  config: config,
  parameters: WatchChainIdParameters<config>,
): WatchChainIdReturnType {
  const { onChange } = parameters;
  return config.subscribe((state) => state.chainId, onChange);
}
