rus = {
    ".-": "а",
    "-...": "б",
    ".--": "в",
    "--.": "г",
    "-..": "д",
    ".": "е",
    "...-": "ж",
    "--..": "з",
    "..": "и",
    ".---": "й",
    "-.-": "к",
    ".-..": "л",
    "--": "м",
    "-.": "н",
    "---": "о",
    ".--.": "п",
    ".-.": "р",
    "...": "с",
    "-": "т",
    "..-": "у",
    "..-.": "ф",
    "....": "х",
    "-.-.": "ц",
    "---.": "ч",
    "----": "ш",
    "--.-": "щ",
    ".--.-.": "ъ",
    "-.--": "ы",
    "-..-": "ь",
    "...-...": "э",
    "..--": "ю",
    ".-.-": "я",
}

# format
# [langs.ru.".-".Layout.lower]
# Layout = "а"
#
# [langs.ru.".-".Layout.upper]
# Layout = "А"

if True:
    for keys, val in rus.items():
        print('[langs.ru."{}".Layout]'.format(keys))
        print('lower.Layout = "{}"'.format(val))

eng = {
    ".-": "a",
    "-...": "b",
    "-.-.": "c",
    "-..": "d",
    ".": "e",
    "..-.": "f",
    "--.": "g",
    "....": "h",
    "..": "i",
    ".---": "j",
    "-.-": "k",
    ".-..": "l",
    "--": "m",
    "-.": "n",
    "---": "o",
    ".--.": "p",
    "--.-": "q",
    ".-.": "r",
    "...": "s",
    "-": "t",
    "..-": "u",
    "...-": "v",
    ".--": "w",
    "-..-": "x",
    "-.--": "y",
    "--..": "z",
}

if True:
    for keys, val in eng.items():
        print('[langs.en."{}".Layout]'.format(keys))
        print('lower.Layout = "{}"'.format(val))


functional = {
    "-.---": (False, "CapsLock"),
    ".-.--": (False, "Backspace"),
    ".--.-": (False, "F9"),
    ".---.-": (False, "Accept"),
    "--.--": (False, "Tab"),
    "......": (True, "."),
    ".-.-.-": (True, ","),
    "..--..": (True, "?"),
    "-....-": (True, "-"),
    ".-..-.": (True, "'"),
    "-.-.-.": (True, ";"),
    "--..--": (True, "!"),
    "-.--.-": (True, "()"),  # key sequence
    "---...": (True, ":"),
    ".-.-.": (True, "+"),
    "---.-": (True, " "),
    ".----": (True, "1"),
    "..---": (True, "2"),
    "...--": (True, "3"),
    "....-": (True, "4"),
    ".....": (True, "5"),
    "-....": (True, "6"),
    "--...": (True, "7"),
    "---..": (True, "8"),
    "----.": (True, "9"),
    "-----": (True, "0"),
}

# format
#
# for code (first False)
# [functional."-.---"]
# Code = "CapsLock"
#
# for layout (first True, one symbol)
# [functional."......".lower]
# Layout = "."
#
# for sequence (first True, more than one symbol)
# [[functional."-.--.-".Sequence]]
# [functional."-.--.-".lower]
# Layout = "("
# [[functional."-.--.-".Sequence]]
# [functional."-.--.-".lower]
# Layout = ")"

if True:
    for seq, (is_not_code, keys) in functional.items():
        if is_not_code:
            if len(keys) == 1:
                print('[functional."{}".Layout]'.format(seq))
                print('lower.Layout = "{}"'.format(keys[0]))
            else:
                for key in keys:
                    print('[[functional."{}".Sequence]]'.format(seq))
                    print('[functional."{}".Sequence.lower]'.format(seq))
                    print('Layout = "{}"'.format(key))
        else:
            print('[functional."{}"]'.format(seq))
            print('Code = "{}"'.format(keys))
