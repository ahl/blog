import Prism from 'prismjs';

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
    'punctuation': /[{}[\];(),.:]/
};
