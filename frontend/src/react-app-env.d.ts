/// <reference types="react-scripts" />

declare module "*.wasm" {
  const value: string;
  export default value;
}

declare module "*.txt" {
  const value: string;
  export default value;
}
