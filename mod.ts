import { instantiate } from "./lib/rs_lib.generated.js";

const { qr_png } = instantiate();

/**
 * Generates a PNG image of a QR code from the given input data.
 */
export function qrPng(data: Uint8Array) {
  return qr_png(data);
}
