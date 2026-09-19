// Extra Prism languages for the syntax highlighter. Anything the
// highlighter can't resolve is emitted RAW AND UNESCAPED by
// eleventy-plugin-syntaxhighlight, so every language used in a code fence
// must resolve here or in Prism itself.
import Prism from 'prismjs';
import PrismComponents from 'prismjs/components/index.js';

// ```console blocks are shell sessions ($-prefixed commands + output).
PrismComponents.silent = true;
PrismComponents('shell-session');
Prism.languages.console = Prism.languages['shell-session'];

// Preformatted non-code text (mail excerpts, quoted comments). An empty
// grammar means no tokens, but unlike the built-in `text` passthrough the
// content still gets HTML-escaped by Prism.highlight().
Prism.languages.plaintext = {};

Prism.languages.dtrace = {
    'comment': {
        pattern: /\/\*[\s\S]*?\*\/|\/\/.*/,
        greedy: true
    },
    'string': {
        pattern: /(["'])(?:\\.|(?!\1)[^\\\r\n])*\1/,
        greedy: true
    },
    'class-name': [
        {
            // Match standalone macros like $1, $2, etc., with a boundary check to avoid matching in larger identifiers
            pattern: /\$[0-9]+\b/,
            alias: 'class-name'
        },
        {
            // Match standalone macros like $target, $foo, etc., with boundary check
            pattern: /\$[a-zA-Z_][a-zA-Z0-9_]*\b/,
            alias: 'class-name'
        }
    ],
    'variable':
    {
        // Match variables like self, this, pid, etc., but exclude cases like pid$...
        pattern: /\b(@|self|this|tid|pid|ppid|probemod|probename|probeprov|probefunc|args)\b/,
        alias: 'variable'
    },
    'keyword': {
        pattern: /\b(?:BEGIN|END|provider|probe|inline|typedef|struct|self|execname|arg[0-9]+)\b/,
        alias: 'keyword'
    },
    'type': {
        pattern: /\b(?:int|char|void|float|double|string|bool|uintptr_t|ushort_t|uchar_t|size_t)\b/,
        alias: 'type'
    },
    'number': /\b(?:0x[\da-f]+|0b[01]+|\d+)\b/i,
    'operator': {
        pattern: /[-+*/%<>=!&|^~?:]/,
        alias: 'operator'
    },
    'punctuation': /[{}[\];(),.:]/,
    'property': {
        pattern: /(^\s*)#\s*[a-zA-Z_]\w*/m,
        lookbehind: true
    },

};
