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
    def __init__(self):
        self.stuff = {}
        self.stuff['syllables'] = [x+y for x in consonants for y in vowels]
        self.stuff['syllables'].extend([y+x for x in consonants for y in vowels])
        self.stuff['syllables'].extend(vowels)
        self.stuff['taken'] = []
        self.stuff['available'] = self.stuff['syllables']
        self.stuff['dictionary'] = {}
        return

    def out(self, string):
        return '/'+string+'/'

    def main(self):
        done = False
        while not done:
            txt = input('>>')
            if txt == 'show taken':
                for i in self.stuff['taken']:
                    print(self.out(i))
            elif txt == 'show available':
                for i in self.stuff['available']:
                    print(self.out(i))
            elif txt == 'show syllables':
                for i in self.stuff['syllables']:
                    print(self.out(i))
            elif txt == 'dictionary':
                self.dictionary()
            elif txt == 'random':
                if not self.stuff['available']:
                    print('All syllables taken.')
                else:
                    print(self.out(choice(self.stuff['available'])))
            elif txt == 'quit':
                print('goodbye')
                done = True
            elif txt in self.stuff['available']:
                prompt = 'What has taken it? \n' \
                         'To cancel input \'cancel\' or \'undo\'\n>>>'
                info = input(prompt)
                if info not in ['undo', 'cancel']:
                    self.update_taken(txt)
                    self.stuff['dictionary'][self.out(txt)] = info
            elif txt in self.stuff['taken']:
                print('Already taken.')
                print('%s : %s' % (self.out(txt), self.lookup_def(txt)))
            else:
                self.output_help()

        pickle.dump(self.stuff, open('data.p', 'wb'))

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
        return self.stuff['dictionary'][self.out(txt)]

    def update_taken(self, txt):
        self.stuff['taken'].append(txt)
        self.stuff['available'].remove(txt)

    def dictionary(self):
        print('WIP')
        return


if __name__ == '__main__':
    # load from pickle
    curr = data()
    with open('data.p', 'rb') as stuff:
        curr.stuff = pickle.load(stuff)
        curr.main()
