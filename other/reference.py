import keyboard as kb
import time
import threading
import winsound


class Lang:
    def __init__(self, *langs, delta_time=1.):
        self.langs = list(map(str, langs))
        self.index = 0
        self.max_index = len(self.langs)
        self.prev_time = time.time()
        self.delta_time = delta_time

    def change(self):
        now = time.time()
        if now - self.prev_time >= self.delta_time:
            self.index += 1
            if self.index == self.max_index:
                self.index = 0
            self.prev_time = now

    def all(self):
        return self.langs

    def __repr__(self):
        return self.langs[self.index]

class FromLangDict(dict):
    def __getitem__(self, key):
        return super().__getitem__(str(key))


def play_short_sound() -> None:
    threading.Thread(target=winsound.Beep, args=(500, 60)).start()


def play_long_sound() -> None:
    threading.Thread(target=winsound.Beep, args=(800, 400)).start()


main_key = 'space'
stop_key = 'ctrl+esc'
change_lang_key = 'ctrl+alt'
time_change_lang = .25
time_erase = .05
time_to_long_sign = .1
time_to_release_letter = .75
prev_condition = False
condition = False
lower = True
sleep_condition = True
letter = ''
lang = Lang("eng", "rus", delta_time=time_change_lang)
prev_time = time.time()
curr_time = time.time()
dict_letters = FromLangDict({
    "rus": {
        ".-": "а", "-...": "б", ".--": "в", "--.": "г",
        "-..": "д", ".": "е", "...-": "ж", "--..": "з",
        "..": "и", ".---": "й", "-.-": "к", ".-..": "л",
        "--": "м", "-.": "н", "---": "о", ".--.": "п",
        ".-.": "р", "...": "с", "-": "т", "..-": "у",
        "..-.": "ф", "....": "х", "-.-.": "ц", "---.": "ч",
        "----": "ш", "--.-": "щ", ".--.-.": "ъ", "-.--": "ы",
        "-..-": "ь", "...-...": "э", "..--": "ю", ".-.-": "я"
    },
    "eng": {
        ".-": "a", "-...": "b", "-.-.": "c", "-..": "d",
        ".": "e", "..-.": "f", "--.": "g", "....": "h",
        "..": "i", ".---": "j", "-.-": "k", ".-..": "l",
        "--": "m", "-.": "n", "---": "o", ".--.": "p",
        "--.-": "q", ".-.": "r", "...": "s", "-": "t",
        "..-": "u", "...-": "v", ".--": "w", "-..-": "x",
        "-.--": "y", "--..": "z"
    },
    "functional": {
        "-.---": "LOWER/UPPER", ".-.--": "BACKSPACE",
        ".--.-": "F9", ".---.-": "ENTER", "--.--": "TAB"
    },
    "can't caps": {
        "......": ".", ".-.-.-": ",", "..--..": "?", "-....-": "-",
        ".-..-.": "'", "-.-.-.": ";", "--..--": "!", "-.--.-": "()",
        "---...": ":", ".-.-.": "+", "---.-": " ", ".----": "1",
        "..---": "2", "...--": "3", "....-": "4", ".....": "5",
        "-....": "6", "--...": "7", "---..": "8", "----.": "9",
        "-----": "0"
    }
})

while not kb.is_pressed(stop_key):
    curr_time = time.time()
    condition = kb.is_pressed(main_key)

    # обработка смены языка
    print(lang, end='\r')
    if kb.is_pressed(change_lang_key):
        lang.change()

    if not sleep_condition and curr_time - prev_time > time_to_release_letter:
        # Обработка нажатой комбинации (что находится в letter)
        sleep_condition = True

        # Стирание
        curr_time_erase = time_erase / len(letter) * 5
        for _ in range(len(letter)):
            time.sleep(curr_time_erase)
            kb.send("backspace")

        if letter in dict_letters["functional"]:
            # Обработка функциональных клавиш (делаю значения заглавными)
            curr = dict_letters["functional"][letter].upper()
            if curr == 'LOWER/UPPER':
                lower = not lower
                print("CHANGE TO", ("LOWER" if lower else "UPPER"))
            else:
                kb.send(curr)
                print("USE:", curr)
            letter = ''
            continue
        elif letter in dict_letters["can't caps"]:
            # Обработка цифр и пунктуации
            kb.write(dict_letters["can't caps"][letter])
            letter = ''
            continue
        elif letter in dict_letters[lang]:
            curr = dict_letters[lang][letter]
            if not lower:
                curr = curr.upper()
            kb.write(curr)
            print("ACCEPTED: '" + curr + "'")
            letter = ''
            continue
        else:
            print("WRONG COMBINATION")
            letter = ''

    if condition != prev_condition:
        # Обработка длительности нажатия (заполняю letter)
        sleep_condition = False
        delta = curr_time - prev_time
        if not condition:
            if delta > time_to_long_sign:
                play_long_sound()
                curr_symbol = '-' if lower else '_'
                curr_symbol_letter = '-'
            else:
                play_short_sound()
                curr_symbol = '.' if lower else '>'
                curr_symbol_letter = '.'
            kb.send('backspace')
            kb.write(curr_symbol)
            letter += curr_symbol_letter
        prev_condition = condition
        prev_time = curr_time
