from random import choice
import pickle

consonants = ['th', 's', 'z', 't', 'd', 'R', 'r', 'l', 'sh', 'hl', 'rr', 'c',
              'j', 'k', 'g', 't\'', 'k\'', 's\'', 'h\'', 'h', 'ts', 'ch', 'ks',
              'dg']
vowels = ['a', 'i', 'u', 'w', 'ai', 'ia', 'uw', 'wu']

# for con in consonants:
#     for vowel in vowels:
#         ret = '/'+con+vowel+'/'+'\n'+'/'+vowel+con+'/'
#         print(ret)
#
# for vowel in vowels:
#     print('/'+vowel+'/')


class data:
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

    def out(self, ret):
        return '/'+ret+'/'

    def main(self):
        done = False
        while not done:
            txt = input('>>')
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
            elif txt == 'find word':
                res = self.word_search()
                if res == 'quit':
                    done = True
            elif txt in self.available:
                prompt = 'What has taken it? \n' \
                         'To cancel input \'cancel\' or \'undo\'\n>>>'
                info = input(prompt)
                if info not in ['undo', 'cancel']:
                    self.update_taken(txt)
                    self.dictionary[self.out(txt)] = info
            elif txt in self.taken:
                print('Already taken.')
                print('%s : %s' % (self.out(txt), self.lookup_def(txt)))
            else:
                self.output_help()
        self.quit()
        return

    def output_help(self):
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
        return self.dictionary[self.out(txt)]

    def update_taken(self, txt):
        self.taken.append(txt)
        self.available.remove(txt)

    def dictionary_menu(self):
        print('Entering Dictionary menu')
        done = False
        while not done:
            txt = input('>>>')
            if txt == 'show dictionary':
                for i, j in self.dictionary.items():
                    print('%s : %s' % (i, j))
            elif txt == 'quit':
                return 'quit'
            elif txt == 'back':
                print('Returning to syllable menu.')
                done = True
            elif txt == 'find word':
                self.word_search()
            elif txt == 'add':
                take = input('What would you like to add?')
                if take not in ['cancel', 'undo']:
                    split = take.split('-')
                    invalid = self.check_syllables(split)
                    collision = self.word_collision(take)
                    if invalid:
                        print('invalid Syllables:' + str(invalid))
                        if '/' in take:
                            print('Do not include slashes, only dashes.')
                    elif self.out(take) in self.dictionary.keys():
                        p = '%s is taken. \n%s : %s' % (take,
                                                        self.out(take),
                                                        self.lookup_def(take))
                        print(p)
                    elif collision:
                        print('%s had a collision with %s.' % (self.out(take),
                                                               collision))
                    else:
                        self.add_def(take)
            elif txt == 'change definition':
                txt = input('What word?\n>>>')
                if self.out(txt) in self.dictionary.keys():
                    n_def = input('What\'s the new definition\n>>>')
                    self.dictionary[self.out(txt)] = [n_def]
            elif self.out(txt) in self.dictionary.keys():
                print('In dictionary.\n%s : %s' % (self.out(txt),
                                                   self.lookup_def(txt)))
        return

    def add_def(self, word):
        text = input('What does %s mean?\n>>>')
        self.dictionary[self.out(word)] = text

    def word_collision(self, word):
        return []

    def check_syllables(self, check):
        ret = []
        for i in check:
            if i not in self.syllables:
                ret.append(i)
        return ret

    def word_search(self):
        return


if __name__ == '__main__':
    # load from pickle
    curr = data(True)
    curr.main()
