import { useEffect, useState } from "react";
import { useAlert, useReadWasmState } from "@gear-js/react-hooks";
import { HexString, ProgramMetadata } from "@gear-js/api";
import meta from "@/assets/meta/vara_go_alpha.meta.txt";
import wasm from "@/assets/meta/alpha_state.meta.wasm";
import { AnyJson } from "@polkadot/types/types";
import { ADDRESS } from "@/consts";

function useBuffer(source: string) {
  const alert = useAlert();

  const [buffer, setBuffer] = useState<Buffer>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => Buffer.from(arrayBuffer))
      .then((result) => setBuffer(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return buffer;
}

function useProgramMetadata(source: string) {
  const alert = useAlert();

  const [metadata, setMetadata] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.text())
      .then((raw) => `0x${raw}` as HexString)
      .then((metaHex) => ProgramMetadata.from(metaHex))
      .then((result) => setMetadata(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return metadata;
}

export function useWasmState<T>(functionName: string, argument: AnyJson) {
  const programMetadata = useProgramMetadata(meta);
  const buffer = useBuffer(wasm);
  console.log(programMetadata);

  return useReadWasmState<T>({
    programId: ADDRESS.CONTRACT as HexString,
    wasm: buffer,
    functionName,
    payload: "0x",
    argument,
    programMetadata,
  });
}
