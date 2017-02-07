from random import choice
import pickle
import tkinter as tk
from Scrollbox import ScrollList

consonants = ['th', 's', 'z', 't', 'd', 'R', 'r', 'l', 'sh', 'hl', 'rr', 'c',
              'j', 'k', 'g', 't\'', 'k\'', 's\'', 'h\'', 'h', 'ts', 'ch', 'ks',
              'dg']
vowels = ['a', 'i', 'u', 'w', 'ai', 'ia', 'uw', 'wu']


class DictionaryApp:
    def __init__(self, from_load):
        self.syllables = []
        self.taken = []
        self.available = []
        self.dictionary = dict()
        self.tags = dict()
        if from_load:
            self.load()
        else:
            self.syllables = [x+y for x in consonants for y in vowels]
            self.syllables.extend([y+x for x in consonants for y in vowels])
            self.syllables.extend(vowels)
            self.taken = []
            self.available = [x for x in self.syllables]
            self.dictionary = {}

        # window setup
        self.root = tk.Tk()
        self.root.title('Draconic Dictionary')

        # setup variables
        self.SylSearchVar = tk.StringVar()
        self.SylSearchVar.trace_variable('w', self.search_syl)

        # what's in the window.
        # All syllables
        self.SylSearchLbl = tk.Label(self.root, text='Syllable Search Box')
        self.SylSearchBox = tk.Entry(self.root,
                                     textvariable=self.SylSearchVar)
        self.SylLbl = tk.Label(self.root, text='Syllable List')
        self.SylScrollList = ScrollList(self.root,
                                        contains=['/%s/' % x for x in self.syllables])
        # Taken syllables
        self.TakenLbl = tk.Label(self.root, text='Taken Syllables')
        self.TakenList = ScrollList(self.root,
                                    contains=['/%s/' % x for x in self.taken])
        # Available Syllables
        self.AvailableLbl = tk.Label(self.root, text='Available Syllables')
        self.AvailableList = ScrollList(self.root,
                                        contains=['/%s/' % x for x in self.available])

        self.set_binds()
        self.grid()
        return

    def set_binds(self):
        self.SylScrollList.bind_listbox('<<ListboxSelect>>', self.syllable_selected)
        return

    def syllable_selected(self, *events):

        return

    def search_syl(self, *events):
        self.SylScrollList.delete(0, tk.END)
        text = self.SylSearchVar.get()
        if text == '':
            for i in self.syllables:
                self.SylScrollList.insert(tk.END, self.out(i))
        else:
            shortList = [x for x in self.syllables if text in x]
            for i in shortList:
                self.SylScrollList.insert(tk.END, self.out(i))
        return

    def grid(self):
        if True:  # Syllable Search box.
            self.SylSearchLbl.grid(row=0, column=0, sticky=tk.W)
            self.SylSearchBox.grid(row=1, column=0)
            self.SylLbl.grid(row=2, column=0, sticky=tk.W)
            self.SylScrollList.grid(row=3, column=0, sticky=tk.N,
                                    rowspan=5, columnspan=1)
            self.TakenLbl.grid(row=0, column=1)
            self.TakenList.grid(row=1, column=1, rowspan=7)
            self.AvailableLbl.grid(row=0, column=2)
            self.AvailableList.grid(row=1, column=2, rowspan=7)
        return

    def load(self):
        temp = pickle.load(open('data.p', 'rb'))
        self.syllables = temp['syllables']
        self.taken = temp['taken']
        self.available = temp['available']
        self.dictionary = temp['dictionary']
        self.tags = temp['tags']
        return

    def quit(self):
        temp = dict()
        temp['dictionary'] = self.dictionary
        temp['syllables'] = self.syllables
        temp['taken'] = self.taken
        temp['available'] = self.available
        temp['tags'] = self.tags
        pickle.dump(temp, open('data.p', 'wb'))

    @staticmethod
    def out(ret):
        return '/'+ret+'/'

    def mainloop(self):
        self.root.mainloop()
        return

    def main(self):
        done = False
        while not done:
            txt = input('>>>')
            if txt == 'show taken':
                for i in self.taken:
                    print(self.out(i))
            elif txt == 'show available':
                for i in self.available:
                    print(self.out(i))
            elif txt == 'show syllables':
                for i in self.syllables:
                    print(self.out(i))
            elif txt == 'dictionary':
                res = self.dictionary_menu()
                if res == 'quit':
                    done = True
            elif txt == 'random':
                if not self.available:
                    print('All syllables taken.')
                else:
                    print(self.out(choice(self.available)))
            elif txt == 'quit':
                print('goodbye')
                done = True
            elif txt == 'reset':
                txt = input('If you are sure input Y.\n>>>')
                if txt == 'Y':
                    self.reset()
            elif txt == 'find word':
                res = self.word_search()
                if res == 'quit':
                    done = True
            elif txt == 'delete':
                self.delete_word()
            elif txt == 'add':
                txt = input('What would you like to add?')
                if txt not in ['cancel', 'undo']:
                    word = txt
                    self.update_taken(word)
                    collision = self.word_collision(txt)
                    if word not in self.syllables:
                        print('Invalid Syllable.')
                    elif txt in self.dictionary.keys():
                        p = '%s is taken. \n%s : %s' % (self.out(txt),
                                                        self.out(txt),
                                                        self.lookup_def(txt))
                        print(p)
                    elif collision:
                        print('%s had a collision with %s.' % (self.out(txt),
                                                               collision))
                    else:
                        self.add_word(word, txt)
            elif txt in self.available:
                print('%s is available.')
            elif txt in self.taken:
                print('Already taken.')
                print('%s : %s' % (self.out(txt), self.lookup_def(txt)))
            else:
                self.main_help()
        self.quit()
        return

    def reset(self):
        self.syllables = [x + y for x in consonants for y in vowels]
        self.syllables.extend([y + x for x in consonants for y in vowels])
        self.syllables.extend(vowels)
        self.available = self.syllables
        self.taken = []
        self.dictionary = dict()
        self.tags = dict()

    def main_help(self):
        print('''Invalid input.
        Valid inputs are:
        show available,
        show taken,
        show syllables,
        any syllable,
        quit,
        dictionary,
        random''')
        return

    def lookup_def(self, txt):
        return self.dictionary[txt]

    def update_taken(self, txt):
        self.taken.append(txt)
        self.available.remove(txt)

    def dictionary_menu(self):
        print('Entering Dictionary menu')
        done = False
        while not done:
            txt = input('>>>')
            if txt == 'show dictionary':
                for i in self.dictionary.keys():
                    self.print_word_info(i)
            elif txt == 'quit':
                return 'quit'
            elif txt == 'back':
                print('Returning to syllable menu.')
                done = True
            elif txt == 'find word':
                ret = self.word_search()
                if ret == 'quit':
                    return ret
            elif txt == 'add':
                txt = input('What would you like to add?')
                if txt not in ['cancel', 'undo']:
                    word = txt
                    split = txt.split('-')
                    invalid = self.check_syllables(split)
                    collision = self.word_collision(txt)
                    if invalid:
                        print('invalid Syllables:' + str(invalid))
                        if '/' in word:
                            print('Do not include slashes, only dashes.')
                    elif txt in self.dictionary.keys():
                        p = '%s is taken. \n%s : %s' % (self.out(txt),
                                                        self.out(txt),
                                                        self.lookup_def(txt))
                        print(p)
                    elif collision:
                        print('%s had a collision with %s.' % (self.out(txt),
                                                               collision))
                    else:
                        self.add_word(word, txt)
            elif txt == 'delete word':
                self.delete_word()
            elif txt == 'show tags':
                self.existing_tags()
            elif txt == 'create tag':
                self.create_tag()
            elif txt == 'add tags':
                txt = input('What word are we tagging.')
                if txt in self.dictionary.keys():
                    self.add_tags(txt)
                else:
                    print('%s does not exist yet.' % self.out(txt))
            elif txt == 'change definition':
                txt = input('What word?\n>>>')
                if txt in self.dictionary.keys():
                    n_def = input('What\'s the new definition\n>>>')
                    self.dictionary[txt] = n_def
            elif txt in self.dictionary.keys():
                self.print_word_info(txt)
            else:
                self.dictionary_help()
        return

    def dictionary_help(self):
        p = 'Invalid command, valid options:\n'
        p += 'change definition\n'
        p += 'show dictionary\n'
        p += 'add tags\n'
        p += 'create tag\n'
        p += 'show tags\n'
        p += 'add\n'
        p += 'quit\n'
        p += 'find word\n'
        p += 'any existing word\n'
        print(p)
        return

    def add_word(self, word, txt):
        self.add_def(txt)
        prompt = input('Are their any tag Y/N')
        if prompt == 'Y':
            self.add_tags(word)
        return

    def delete_word(self):
        word = input('What do you want to delete?\n>>>')
        if word in self.dictionary.keys():
            self.dictionary.pop(word)
        if word in self.taken:
            self.taken.remove(word)
            self.available.append(word)
        print('%s removed.' % word)
        return

    def print_word_info(self, word):
        fin = '%s\n Definition: %s\nTags: %s' % (self.out(word),
                                                 self.lookup_def(word),
                                                 self.get_tags(word))
        print(fin)
        return

    def get_tags(self, word):
        tags = [x for x in self.tags.keys() if word in self.tags[x]]
        return str(tags)

    def existing_tags(self):
        if self.tags:
            print('Current Tags:')
            for i in self.tags.keys():
                print(i)
        else:
            print('No tags currently exist.')

    def create_tag(self):
        self.existing_tags()
        tag = input('What is the new tag? Separate them with ,\n>>>')
        tags = tag.split(', ')
        print(tags)
        for ta in tags:
            if ta in self.tags.keys():
                print('Tag %s already exists.' % ta)
            else:
                self.tags[ta] = []
            print('%s added.' % ta)
        return

    def add_tags(self, word):
        print('Current Tags: \n%s' % self.get_tags(word))
        self.existing_tags()
        txt = input('Separate tags by Commas.\n>>>')
        tags = txt.split(', ')
        for i in tags:
            if i not in self.tags.keys():
                print('Tag %s does not exist.' % i)
            else:
                self.tags[i].append(word)
        return

    def add_def(self, word):
        text = input('What does %s mean?\n>>>' % word)
        self.dictionary[word] = text

    def word_collision(self, word):
        return []

    def check_syllables(self, check):
        ret = []
        for i in check:
            if i not in self.syllables:
                ret.append(i)
        return ret

    def word_search(self):
        print('In word search menu.')
        done = False
        while not done:
            txt = input('>>>')
            if txt == 'quit':
                return 'quit'
        return


if __name__ == '__main__':
    # load from pickle
    curr = DictionaryApp(True)
    curr.mainloop()
