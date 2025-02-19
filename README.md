# The idea of mnemonic sentences for BitTorrent info-hashes

## Rationale
Info hashes are random symbols, they are hard to remember.
In addition to this, most of the heavily censored platforms remove such links (lately with the help of AI). Usefulness of such links for the community is never given an option.

## Introduction
Mnemonic words could potentially describe the hashes.

We take a language based dictionary approach and respectively map its words to hashes. Doesn't matter which language it is.
This approach uses English with borrowed words (German, French, Russian, Latin), with hash length division by 4.

_Why four? Wouldn't using wider window shorten the mnemonic?_

The hash could consist of 16 symbols (a-f, 0-9) and it's probability for four symbols is 16^4=65536 words. Making hash code length 5 would give us more than a million words, there are no common languages containing such amount of unique words.
The hashing algorithm used here is Keccak (SHAKE256), since it allows us to configure digest output length.

# Example usage

Mnemonic encoding:
```bash
> mnm encode f52b4103641b696edafecb1b2d64711f043fdffc # v1:
# magnet:?xt=urn:btih:songent-varicella-definissent-reformation-restaging-pilotin-blackstock-glick-tuilleries-complaisantly

> mnm encode 77cbb5e57fa15bf28e81d7d6fc7e50b76b76178ce625ec68d835d3b1866c3b05 # v2:
# magnet:?xt=urn:btmh:1220holders-imitating-coursebook-extraumbilical-cyclones-mordern-accetta-gamelion-tagadur-catini-brook-fixler-saleswork-leukemia-runs-amongst

> mnm encode 8d56d222fe06c09b3b57ecbd4703db2bbc968a63 pick # Lets you to choose your own words
```
Mnemonic decoding:
```bash
> mnm decode songent-varicella-definissent-reformation-restaging-pilotin-blackstock-glick-tuilleries-complaisantly
# magnet:?xt=urn:btih:f52b4103641b696edafecb1b2d64711f043fdffc
```

# Credits
[@fmerg](https://github.com/fmerg) and his contributors for the SHAKE256 implementation.

[@david47k](https://github.com/david47k) for the dictionary.

The code here is largely undocumented.
