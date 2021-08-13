# Base64ct PoC

The code uses two short base64-encoded strings to demonstrate the discrepancy
in how base64ct handles the last character. Base64 is chosen as the point of
comparison because it handles *this* issue correctly IMO.

The two strings:

- Mi, which is `001100 100010`, or split "on the byte," `00110010 0010`.

- Mg, which is `001100 100000`, or split "on the byte," `00110010 0000`.

The former contains trailing bits that are non-zero, thus shouldn't be ignored,
but are ignored by base64ct.

The latter has only zero-valued trailing bits, so the bits are safely ignored
by both base64 and base64ct.

<https://en.wikipedia.org/wiki/Base64#Base64_table>
