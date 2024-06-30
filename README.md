# Qr Png

This module provides functionality to generate a PNG image of a QR code from
given input data.

## Examples

**Example 1**

```ts
import { qrPng } from "@sigmasd/qrpng";

const data = new TextEncoder().encode("Hello, world!");
const qrCodePng = qrPng(data);
console.log(qrCodePng);
```
