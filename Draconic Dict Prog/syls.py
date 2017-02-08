from random import choice
import pickle
import tkinter as tk
from tkinter import ttk
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

        self.Tabs = ttk.Notebook(self.root)

        # SylTab Section
        self.SylTab = tk.Frame(self.Tabs)
        # setup variables
        self.SylSearchVar = tk.StringVar()
        self.SylSearchVar.trace_variable('w', self.search_syl)
        self.ChosenSylVar = tk.StringVar()
        self.CreateTagVar = tk.StringVar()
        self.CreateTagVar.trace_variable('w', self.search_tags)
        self.DictSearchVar = tk.StringVar()
        self.DictSearchVar.trace_variable('w', self.search_dictionary)

        #### what's in the window.
        #### All syllables
        self.SylSearchLbl = tk.Label(self.SylTab, text='Syllable Search Box')
        self.SylSearchBox = tk.Entry(self.SylTab,
                                     textvariable=self.SylSearchVar)
        self.SylLbl = tk.Label(self.SylTab, text='Syllable List')
        self.SylScrollList = ScrollList(self.SylTab,
                                        contains=['/%s/' % x for x in self.syllables])
        # Taken syllables
        self.TakenLbl = tk.Label(self.SylTab, text='Taken Syllables')
        self.TakenList = ScrollList(self.SylTab,
                                    contains=['/%s/' % x for x in self.taken])
        # Available Syllables
        self.AvailableLbl = tk.Label(self.SylTab, text='Available Syllables')
        self.AvailableList = ScrollList(self.SylTab,
                                        contains=['/%s/' % x for x in self.available])
        # Random buttons
        self.RandomButtons = tk.Frame(self.SylTab)
        self.RandomSylButton = tk.Button(self.RandomButtons, text='Random Syllable',
                                         command=self.random_syl)
        self.RandomTakenButton = tk.Button(self.RandomButtons, text='Random Taken Syllable',
                                           command=self.random_taken)
        self.RandomAvailableButton = tk.Button(self.RandomButtons, text='Random Available Syllable',
                                               command=self.random_available)
        # Chosen Syllable info.
        self.ChosenFrame = tk.Frame(self.SylTab)
        self.ChosenLbl = tk.Label(self.ChosenFrame, text='Chosen Syllable Info')
        self.ChosenSylBox = tk.Entry(self.ChosenFrame, state='readonly',
                                     textvariable=self.ChosenSylVar,
                                     readonlybackground='white')
        self.ChosenSylDefLbl = tk.Label(self.ChosenFrame, text='Definition')
        self.ChosenSylDef = tk.Text(self.ChosenFrame, wrap=tk.WORD)
        # Chosen Syllable Create Destroy buttons
        self.SaveSylButton = tk.Button(self.ChosenFrame,
                                       text='Add Syllable Info',
                                       command=self.SaveSyl)
        self.DeleteSylButton = tk.Button(self.ChosenFrame,
                                         text='Clear This Syllable',
                                         command=self.DeleteSyl)
        ## Tag editor
        # Chosen List
        self.ChosenSylTagsLbl = tk.Label(self.ChosenFrame, text='Tags')
        self.ChosenSylTags = ScrollList(self.ChosenFrame)
        # Add remove Tag Buttons
        self.TagAddRemoveBox = tk.Frame(self.ChosenFrame)
        self.AddTagButton = tk.Button(self.TagAddRemoveBox, text='<<',
                                      command=self.add_tag)
        self.RemoveTagButton = tk.Button(self.TagAddRemoveBox, text='>>',
                                         command=self.remove_tag)
        # All Tag List
        self.AllTagsLbl = tk.Label(self.ChosenFrame, text='Available Tags')
        self.AllTagsList = ScrollList(self.ChosenFrame,
                                      contains=self.tags.keys())
        # Create Tag Box
        self.CreateTagBox = tk.Entry(self.ChosenFrame,
                                     textvariable=self.CreateTagVar)
        self.CreateDeleteTagBox = tk.Frame(self.ChosenFrame)
        self.CreateTagButton = tk.Button(self.CreateDeleteTagBox,
                                         text='Add Tag',
                                         command=self.create_tag)
        self.DeleteTagButton = tk.Button(self.CreateDeleteTagBox,
                                         text='Delete Tag',
                                         command=self.delete_tag)


        #### Dictionary Tab
        self.DictTab = tk.Frame(self.Tabs)
        # Current Words
        self.DictLbl = tk.Label(self.DictTab, text='Current Words')
        self.DictList = ScrollList(self.DictTab,
                                   ['/%s/' % x for x in self.dictionary.keys()])
        self.DictSearchLbl = tk.Label(self.DictTab, text='Search Box')
        self.DictSearchBox = tk.Entry(self.DictTab, textvariable=self.DictSearchVar)

        #### Tab Setups
        self.Tabs.add(self.SylTab, text='Syllable Tab')
        self.Tabs.add(self.DictTab, text='Dictionary Tab')
        self.Tabs.grid(row=0, column=0)

        self.set_binds()
        self.SylGrid()
        self.DictGrid()
        return

    def DictGrid(self):
        self.DictLbl.grid(row=0, column=0)
        self.DictList.grid(row=1, column=0)
        self.DictSearchLbl.grid(row=2, column=0)
        self.DictSearchBox.grid(row=3, column=0)

    def SylGrid(self):
        self.SylSearchLbl.grid(row=0, column=0, sticky=tk.W)
        self.SylSearchBox.grid(row=1, column=0)
        self.SylLbl.grid(row=2, column=0, sticky=tk.W)
        self.SylScrollList.grid(row=3, column=0, sticky=tk.N,
                                rowspan=5, columnspan=1)
        # Taken Syllables
        self.TakenLbl.grid(row=0, column=1)
        self.TakenList.grid(row=1, column=1, rowspan=7)
        # Available Syllables
        self.AvailableLbl.grid(row=0, column=2)
        self.AvailableList.grid(row=1, column=2, rowspan=7)
        # Random Buttons
        self.RandomButtons.grid(row=3, column=3)
        self.RandomSylButton.grid(row=0, column=0)
        self.RandomTakenButton.grid(row=1, column=0)
        self.RandomAvailableButton.grid(row=2, column=0)
        # Chosen Frame Data
        self.ChosenFrame.grid(row=8, column=0, columnspan=9)
        # Chosen Syllable Box
        self.ChosenLbl.grid(row=0, column=0)
        self.ChosenSylBox.grid(row=1, column=0, sticky=tk.N)
        # Definition
        self.ChosenSylDefLbl.grid(row=0, column=1)
        self.ChosenSylDef.grid(row=1, column=1, columnspan=2)
        # Syllable create/destroy
        self.SaveSylButton.grid(row=4, column=1)
        self.DeleteSylButton.grid(row=4, column=2)
        # Syl Tags List
        self.ChosenSylTagsLbl.grid(row=0, column=4)
        self.ChosenSylTags.grid(row=1, column=4, rowspan=2, sticky=tk.N+tk.S)
        # Add remove tag buttons
        self.TagAddRemoveBox.grid(row=1, column=5)
        self.AddTagButton.grid(row=0, column=0, sticky=tk.S)
        self.RemoveTagButton.grid(row=1, column=0, sticky=tk.N)
        # All tags list
        self.AllTagsLbl.grid(row=0, column=6)
        self.AllTagsList.grid(row=1, column=6, rowspan=2, sticky=tk.N+tk.S)
        self.CreateTagBox.grid(row=3, column=6)
        # Create Delete Tag buttons
        self.CreateDeleteTagBox.grid(row=4, column=6)
        self.CreateTagButton.grid(row=0, column=0)
        self.DeleteTagButton.grid(row=0, column=1)
        return

    def search_dictionary(self, *events):
        val = self.DictSearchVar.get().replace('/', '')
        self.DictList.delete(0, tk.END)
        for i in self.dictionary.keys():
            if val in i:
                self.DictList.insert(tk.END, self.out(i))
        return

    def random_syl(self, *events):
        res = choice(self.syllables)
        self.ChosenSylVar.set(self.out(res))
        self.update_chosen_syl(self.out(res), res)
        return

    def random_taken(self, *events):
        res = choice(self.taken)
        self.ChosenSylVar.set(self.out(res))
        self.update_chosen_syl(self.out(res), res)
        return

    def random_available(self, *events):
        res = choice(self.available)
        self.ChosenSylVar.set(self.out(res))
        self.update_chosen_syl(self.out(res), res)
        return

    def update_taken_available(self):
        self.TakenList.delete(0, tk.END)
        self.AvailableList.delete(0, tk.END)
        for i in self.taken:
            self.TakenList.insert(tk.END, self.out(i))
        for i in self.available:
            self.AvailableList.insert(tk.END, self.out(i))
        return

    def search_tags(self, *event):
        self.AllTagsList.delete(0, tk.END)
        text = self.CreateTagVar.get()
        if text == '':
            for i in self.tags.keys():
                self.AllTagsList.insert(tk.END, i)
        else:
            shortList = [x for x in self.tags.keys() if text in x]
            for i in shortList:
                self.AllTagsList.insert(tk.END, i)
        return

    def set_binds(self):
        self.SylScrollList.bind_listbox('<<ListboxSelect>>', self.syllable_selected)
        self.TakenList.bind_listbox('<<ListboxSelect>>', self.taken_selected)
        self.AvailableList.bind_listbox('<<ListboxSelect>>', self.available_selected)
        return

    def delete_tag(self, *event):
        tag = self.CreateTagVar.get()
        if tag in self.tags.keys():
            self.tags.pop(tag)
            self.AllTagsList.delete(0, 0)
        return

    def update_chosen_syl(self, val, syl):
        self.ChosenSylVar.set(val)
        self.ChosenSylDef.delete('0.0', tk.END)
        if syl in self.taken:
            self.ChosenSylDef.insert(tk.END, self.lookup_def(syl))
        self.ChosenSylTags.delete(0, tk.END)
        for i in self.get_tags(syl):
            self.ChosenSylTags.insert(tk.END, i)
        return

    def syllable_selected(self, *events):
        val = self.SylScrollList.curitem()
        syl = val.replace('/', '')
        self.update_chosen_syl(val, syl)
        return

    def taken_selected(self, *events):
        val = self.TakenList.curitem()
        syl = val.replace('/', '')
        self.update_chosen_syl(val, syl)
        return

    def available_selected(self, *events):
        val = self.AvailableList.curitem()
        syl = val.replace('/', '')
        self.update_chosen_syl(val, syl)
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

    def SaveSyl(self, *events):
        syl = self.ChosenSylVar.get().replace('/', '')
        if syl not in self.taken:
            self.taken.append(syl)
        if syl in self.available:
            self.available.remove(syl)
        self.dictionary[syl] = self.ChosenSylDef.get('0.0', tk.END)
        for i in self.ChosenSylTags.get(0, tk.END):
            if syl not in self.tags[i]:
                self.tags[i].append(syl)
        self.update_taken_available()
        return

    def DeleteSyl(self, *events):
        syl = self.ChosenSylVar.get().replace('/', '')
        if syl in self.taken:
            self.taken.remove(syl)
        if syl not in self.available:
            self.available.append(syl)
        if syl in self.dictionary.keys():
            self.dictionary.pop(syl)
        for i in self.ChosenSylTags.get(0, tk.END):
            if syl in self.tags[i]:
                self.tags[i].remove(syl)
        self.update_taken_available()
        self.ChosenSylDef.delete('0.0', tk.END)
        self.ChosenSylTags.delete(0, tk.END)
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
        self.load()
        self.root.mainloop()
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

    def lookup_def(self, txt):
        return self.dictionary[txt]

    def get_tags(self, word):
        tags = [x for x in self.tags.keys() if word in self.tags[x]]
        return tags

    def create_tag(self, *events):
        tag = self.CreateTagVar.get()
        if tag not in self.tags.keys():
            self.tags[tag] = []
            self.AllTagsList.insert(tk.END, tag)
        return

    def add_tag(self, *events):
        tag = self.AllTagsList.curitem()
        word = self.ChosenSylVar.get().replace('/', '')
        if word not in self.tags[tag]:
            self.tags[tag].append(word)
        self.update_chosen_tags(word)
        return

    def update_chosen_tags(self, word):
        self.ChosenSylTags.delete(0, tk.END)
        for i in self.get_tags(word):
            self.ChosenSylTags.insert(tk.END, i)
        return

    def remove_tag(self, *events):
        tag = self.ChosenSylTags.curitem()
        word = self.ChosenSylVar.get().replace('/', '')
        if word in self.tags[tag]:
            self.tags[tag].remove(word)
        self.update_chosen_tags(word)
        return

    def word_collision(self, word):
        return []


if __name__ == '__main__':
    # load from pickle
    curr = DictionaryApp(True)
    curr.mainloop()
