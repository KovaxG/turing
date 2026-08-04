# Turing interpreter

I implemented this interpreter to better understand how Turing machines work. I basing everything on the original paper by Alan Turing called "On Computable Numbers, with an Application to the Entscheidungsproblem" published in 1936.

## 𝔗𝔥𝔢 𝔊𝔬𝔱𝔥𝔦𝔠 𝔄𝔩𝔭𝔥𝔞𝔟𝔢𝔱

It was fashionable to use Gothic letters when Turing's article was written. Some characters might look a bit different from the font used in the article, for example 𝔨 is more similar to 𝔣.

**L** stands for the latin alphabet and **𝔊** stands for the gothic alphabet.

| L |  𝔊  || L |  𝔊 || L |  𝔊  || L | 𝔊  || L |  𝔊  |
|---|-----|-|---|----|-|---|-----|-|---|----|-|---|-----|
| a | 𝔄 𝔞 || g | 𝔊 𝔤 || m | 𝔐 𝔪 || s | 𝔖 𝔰 ||  y | 𝔜 𝔶 |
| b | 𝔅 𝔟 || h | ℌ 𝔥 || n | 𝔑 𝔫 || t | 𝔗 𝔱 ||  z | ℨ 𝔷 |
| c | ℭ 𝔠 || i | ℑ 𝔦 || o | 𝔒 𝔬 || u | 𝔘 𝔲 ||   |     |
| d | 𝔇 𝔡 || j | 𝔍 𝔧 || p | 𝔓 𝔭 || v | 𝔙 𝔳 ||   |    |
| e | 𝔈 𝔢 || k | 𝔎 𝔨 || q | 𝔔 𝔮 || w | 𝔚 𝔴 ||  |    |
| f | 𝔉 𝔣 || l | 𝔏 𝔩 || r | ℜ 𝔯 || x | 𝔛 𝔵 ||     |   |
