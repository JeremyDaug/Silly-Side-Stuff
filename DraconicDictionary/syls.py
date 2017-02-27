import pickle
import tkinter as tk
from random import choice
from tkinter import ttk
import tkinter.messagebox
from DraconicDictionary.Searchlistbox import SearchListBox

from DraconicDictionary.Scrollbox import Scrollbox

consonants = ['th', 's', 'z', 't', 'd', 'R', 'r', 'l', 'sh', 'hl', 'rr', 'c',
              'j', 'k', 'g', 't\'', 'k\'', 's\'', 'h\'', 'h', 'ts', 'ch', 'ks',
              'dg']
vowels = ['a', 'i', 'u', 'w', 'ai', 'ia', 'uw', 'wu']
WordAffixOrder = ['Grammar Affix', 'Prepositional Flag', 'Prepositional/Clause Affix',
                  'Factuality Affix', 'Negative Affix',
                  'Intensity Affix', 'Progressive Affix', 'Root',
                  'Recurrence Affix', 'Temporal Affix', 'Numeric Affix',
                  'Gender Affix']


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
        self.ChosenSylVar = tk.StringVar()
        self.CreateTagVar = tk.StringVar()
        self.CreateTagVar.trace_variable('w', self.search_tags)

        #### what's in the window.
        #### All syllables
        self.SylSearchListBox = SearchListBox(self.SylTab, label='Syllable Search Box',
                                              parent_list=[self.out(x) for x in self.syllables])
        # Taken syllables
        self.TakenLbl = tk.Label(self.SylTab, text='Taken Syllables')
        self.TakenList = Scrollbox(self.SylTab,
                                   contains=['/%s/' % x for x in self.taken])
        # Available Syllables
        self.AvailableLbl = tk.Label(self.SylTab, text='Available Syllables')
        self.AvailableList = Scrollbox(self.SylTab,
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
        self.ChosenSylTags = Scrollbox(self.ChosenFrame)
        # Add remove Tag Buttons
        self.TagAddRemoveBox = tk.Frame(self.ChosenFrame)
        self.AddTagButton = tk.Button(self.TagAddRemoveBox, text='<<',
                                      command=self.add_tag)
        self.RemoveTagButton = tk.Button(self.TagAddRemoveBox, text='>>',
                                         command=self.remove_tag)
        # All Tag List
        self.AllTagsLbl = tk.Label(self.ChosenFrame, text='Available Tags')
        self.AllTagsList = Scrollbox(self.ChosenFrame,
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
        self.DictChosenVar = tk.StringVar()
        self.DictTab = tk.Frame(self.Tabs)
        # Current Words
        self.ExistingWordSearchBox = SearchListBox(self.DictTab, label='Current Words',
                                                   parent_list=['/%s/' % x for x in self.dictionary.keys()])
        # Chosen Words
        self.ChosenWordLbl = tk.Label(self.DictTab, text='Current Word')
        self.ChosenWordBox = tk.Entry(self.DictTab, textvariable=self.DictChosenVar, state='readonly',
                                      readonlybackground='white')
        # Definition
        self.ChosenWordDef = tk.Text(self.DictTab)
        # Tags
        self.ChosenWordTags = Scrollbox(self.DictTab)
        # Add remove Tags
        self.DictTagButtonBox = tk.Frame(self.DictTab)
        self.WordAddTagButton = tk.Button(self.DictTagButtonBox, text='<<', command=self.add_word_tag)
        self.WordDeleteTagButton = tk.Button(self.DictTagButtonBox, text='>>', command=self.delete_word_tag)
        # All tags
        self.DictAllTagsSearch = SearchListBox(self.DictTab, label='All tags',
                                               parent_list=self.tags.keys())
        # Save Clear Word Data
        self.SaveClearBox = tk.Frame(self.DictTab)
        self.SaveWordButton = tk.Button(self.SaveClearBox, text='Save Definition', command=self.save_word_def)
        self.ClearWordButton = tk.Button(self.SaveClearBox, text='Delete Word from Dictionary',
                                         command=self.delete_word)
        # Word Crafting
        self.TypeBoxVar = tk.StringVar()
        self.ErrorVar = tk.StringVar()
        self.TypeBoxVar.trace_variable('w', self.type_box_checker)
        self.ListLabelVar = tk.StringVar()
        self.ListTypeVar = tk.StringVar()
        self.ListTypeVar.set('Unused Syllables')
        self.WordCraftBox = tk.Frame(self.DictTab)
        self.TypeBoxLbl = tk.Label(self.WordCraftBox, text='New Word Box')
        self.TypeBox = tk.Entry(self.WordCraftBox,
                                textvariable=self.TypeBoxVar)
        self.ErrorLbl = tk.Label(self.WordCraftBox, textvariable=self.ErrorVar)
        self.AddWordButton = tk.Button(self.WordCraftBox, text='View Word',
                                       command=self.dict_add_word)
        self.OptionsBox = tk.Frame(self.WordCraftBox)
        self.OptionsSearchBox = SearchListBox(self.OptionsBox, label='Syllables to add.',
                                              parent_list=self.get_requested_syls())
        self.ListTypeCombo = ttk.Combobox(self.OptionsBox, textvariable=self.ListTypeVar, state='readonly')
        self.ListTypeCombo['values'] = ['Unused Syllables', 'Used Syllables', 'Atomic Words']
        self.RandomUnusedSyl = tk.Button(self.OptionsBox, text='Random Unused Syllable',
                                         command=self.dict_random_unused_syl)
        self.RandomNonAffix = tk.Button(self.OptionsBox, text='Random Non Affix Syllable',
                                        command=self.dict_random_non_affix)

        # Definition search box
        self.DefSearchListBox = SearchListBox(self.DictTab, label='Definition Search Box',
                                              parent_list=[self.out(x) for x in self.dictionary.keys()],
                                              search_func=self.def_search_defs)

        # Word Exploration Tab
        self.ExplorationTab = tk.Frame(self.Tabs)
        # word selection box
        self.ExplorationSearchBox = SearchListBox(self.ExplorationTab,
                                                  label='Current Words',
                                                  parent_list=[self.out(x) for x in self.dictionary.keys()])
        # Current Explore Word List
        self.CurrentWordVar = tk.StringVar()
        self.CurrentWordLbl = tk.Label(self.ExplorationTab, text='Current Word')
        self.CurrentWordBox = tk.Entry(self.ExplorationTab, textvariable=self.CurrentWordVar,
                                       state='readonly', readonlybackground='white')
        # Exploration Search by Def
        self.ExpSearchByDef = SearchListBox(self.ExplorationTab,
                                            label='Search By Definition',
                                            parent_list=[self.out(x) for x in self.dictionary.keys()],
                                            search_func=self.exp_search_defs)
        # Exploration Box
        self.ExplorationBox = tk.Text(self.ExplorationTab)
        # Tags
        self.ExplorationTags = Scrollbox(self.ExplorationTab)

        #### Tab Setups
        self.Tabs.add(self.SylTab, text='Syllable Tab')
        self.Tabs.add(self.DictTab, text='Dictionary Tab')
        self.Tabs.add(self.ExplorationTab, text='Exploration Tab')
        self.Tabs.grid(row=0, column=0)

        self.set_binds()
        self.SylGrid()
        self.DictGrid()
        self.ExploreGrid()
        return

    def explore_word(self, *events):
        word = self.ExplorationSearchBox.get_curitem().replace('/', '')
        return self.update_explore_info(word)

    def exp_select_word(self, *events):
        word = self.DefSearchListBox.get_curitem().replace('/', '')
        return self.update_explore_info(word)

    def update_explore_info(self, word):
        self.CurrentWordVar.set(self.out(word))
        self.ExplorationBox.delete('0.0', tk.END)
        if word in self.dictionary.keys():
            self.ExplorationBox.insert(tk.END, self.dictionary[word])
        self.ExplorationTags.delete(0, tk.END)
        for i in self.get_tags(word):
            self.ExplorationTags.insert(tk.END, i)
        return

    def ExploreGrid(self):
        self.ExplorationSearchBox.grid(row=0, column=0)
        self.CurrentWordLbl.grid(row=1, column=0)
        self.CurrentWordBox.grid(row=2, column=0, sticky=tk.N)
        self.ExplorationBox.grid(row=2, column=1)
        self.ExplorationTags.grid(row=2, column=2, sticky=tk.N+tk.S)
        self.ExpSearchByDef.grid(row=0, column=1, rowspan=2)
        return

    def search_defs(self, var=None):
        if not isinstance(var, SearchListBox):
            raise TypeError('Var must be of type SearchListBox')
        search = var.mysearchvar.get()
        var.mylist.delete(0, tk.END)
        if not search:
            return
        for i in self.dictionary.keys():
            if search.lower() in self.dictionary[i].lower():
                var.mylist.insert(tk.END, self.out(i))
        return

    def exp_search_defs(self, *events):
        return self.search_defs(self.ExpSearchByDef)

    def def_search_defs(self, *events):
        return self.search_defs(self.DefSearchListBox)

    def show_word(self, *events):
        selected = self.DefSearchListBox.get_curitem().replace('/', '')
        self.DictChosenVar.set(selected)
        self.ChosenWordDef.delete('0.0', tk.END)
        self.ChosenWordDef.insert(tk.END, self.dictionary[selected])
        self.ChosenWordTags.delete(0, tk.END)
        for i in self.get_tags(selected):
            self.ChosenWordTags.insert(tk.END, i)
        return

    def dict_random_non_affix(self, *events):
        group = [i for i in self.available]
        non_affixes = []
        for i in group:
            not_affix = True
            for tag in self.get_tags(i):
                if 'Affix' in tag or 'Flag' in tag:
                    not_affix = False
                    break
            if not_affix:
                non_affixes.append(i)
        word = self.TypeBoxVar.get()
        if word:
            if word[-1] != '-':
                word += '-'
        word += choice(non_affixes)
        self.TypeBoxVar.set(word)
        return

    def dict_random_unused_syl(self, *events):
        word = self.TypeBoxVar.get()
        if word:
            if word[-1] != '-':
                word += '-'
        word += choice([i for i in self.available])
        self.TypeBoxVar.set(word)
        return

    def WordCraftBoxGrid(self):
        self.WordCraftBox.grid(row=0, column=1, rowspan=4)
        self.TypeBoxLbl.grid(row=0, column=0)
        self.TypeBox.grid(row=1, column=0)
        self.ErrorLbl.grid(row=3, column=0)
        self.AddWordButton.grid(row=2, column=0)
        self.OptionsBox.grid(row=1, column=2, rowspan=4)
        self.OptionsSearchBox.grid(row=0, column=0, rowspan=1)
        self.ListTypeCombo.grid(row=0, column=3)
        self.RandomUnusedSyl.grid(row=0, column=4)
        self.RandomNonAffix.grid(row=1, column=4)
        return

    def change_list_type(self, *events):
        self.TypeBoxVar.set('')
        self.OptionsSearchBox.update_list(self.get_requested_syls())
        return

    def get_requested_syls(self):
        type = self.ListTypeVar.get()
        if type == 'Unused Syllables':
            return [self.out(syl) for syl in self.syllables if syl not in self.dictionary.keys()]
        elif type == 'Used Syllables':
            return [self.out(syl) for syl in self.syllables if syl in self.dictionary.keys()]
        elif type == 'Atomic Words':
            temp = [i for i in self.dictionary.keys() if '-' not in i]
            ret = []
            for i in temp:
                affix = False
                for tag in self.get_tags(i):
                    if 'Affix' in tag or 'Flag' in tag:
                        affix = True
                        break
                if not affix:
                    ret.append(self.out(i))
            return ret

    def dict_add_word(self, *events):
        word = self.TypeBoxVar.get().strip('/')
        res, root = self.word_variant(word)
        if word in self.dictionary.keys():
            self.DictChosenVar.set(self.out(word))
            self.ChosenWordDef.delete('0.0', tk.END)
            if word in self.dictionary.keys():
                self.ChosenWordDef.insert(tk.END, self.lookup_def(word))
            self.ChosenWordTags.delete(0, tk.END)
            for i in self.get_tags(word):
                self.ChosenWordTags.insert(tk.END, i)
        elif res == 'Variant':
            self.DictChosenVar.set(self.out(word))
            self.ChosenWordDef.delete('0.0', tk.END)
            if root in self.dictionary.keys():
                self.ChosenWordDef.insert(tk.END, 'Word Variant')  # TODO LATER create a function to explain a variant.
            self.ChosenWordTags.delete(0, tk.END)
        else:
            self.DictChosenVar.set(self.out(word))
            self.ChosenWordDef.delete('0.0', tk.END)
            self.ChosenWordTags.delete(0, tk.END)
        return

    def type_box_checker(self, *events):
        word = self.TypeBoxVar.get().replace('/', '')
        if not word:
            self.TypeBox.config(bg='white')
            self.ErrorVar.set('')
            return
        syls = word.split('-')
        for i in syls:
            if i not in self.syllables:
                self.TypeBox.config(bg='red')
                self.ErrorVar.set('Invalid Syllable %s' % i)
                return
        collision, root = self.word_collision(word)
        if collision == 'Collision':
            self.TypeBox.config(bg='red')
            self.ErrorVar.set('Word already taken')
        elif collision == 'Variant' and root in self.dictionary.keys():
            self.TypeBox.config(bg='yellow')
            self.ErrorVar.set('Word is variant of another word.')
        elif collision == 'Variant' and root not in self.dictionary.keys():
            self.TypeBox.config(bg='yellow')
            self.ErrorVar.set('Word is variant, but %s is available.' % root)
        elif collision == 'Improper Affix Order':
            self.TypeBox.config(bg='yellow')
            self.ErrorVar.set('Word uses improper affixes, open but be warned.')
        elif collision == 'Duplicate Affixes':
            self.TypeBox.config(bg='yellow')
            self.ErrorVar.set('Duplicate Affixes, viable, but questionable.')
        elif collision == 'Lone Flag':
            self.TypeBox.config(bg='yellow')
            self.ErrorVar.set('Lone prepositional flag, can be valid.')
        else:
            self.TypeBox.config(bg='white')
            self.ErrorVar.set('')
        return

    def set_binds(self):
        self.SylSearchListBox.bind_listbox('<<ListboxSelect>>', self.syllable_selected)
        self.TakenList.bind_listbox('<<ListboxSelect>>', self.taken_selected)
        self.AvailableList.bind_listbox('<<ListboxSelect>>', self.available_selected)
        self.ExistingWordSearchBox.bind_listbox('<<ListboxSelect>>', self.word_selected)
        self.OptionsSearchBox.bind_listbox('<<ListboxSelect>>', self.add_to_craft)
        self.ListTypeCombo.bind('<<ComboboxSelected>>', self.change_list_type)
        self.DefSearchListBox.bind_listbox('<<ListboxSelect>>', self.show_word)
        self.ExplorationSearchBox.bind_listbox('<<ListboxSelect>>', self.explore_word)
        self.ExpSearchByDef.bind_listbox('<<ListboxSelect>>', self.exp_select_word)
        return

    def add_to_craft(self, *events):
        word = self.TypeBoxVar.get()
        if word:
            if word[-1] != '-':
                word += '-'
        word += self.OptionsSearchBox.get_curitem().replace('/', '')
        self.TypeBoxVar.set(word)
        return

    def help(self):
        return 'To delete a flag you must type it\'s full name and hit delete.'

    def DictGrid(self):
        self.ExistingWordSearchBox.grid(row=0, column=0, rowspan=4)
        self.ChosenWordLbl.grid(row=4, column=0)
        self.ChosenWordBox.grid(row=5, column=0, sticky=tk.N)
        self.ChosenWordDef.grid(row=5, column=1)
        self.ChosenWordTags.grid(row=5, column=2, sticky=tk.N+tk.S)
        self.DictTagButtonBox.grid(row=5, column=3)
        self.WordAddTagButton.grid(row=0, column=0)
        self.WordDeleteTagButton.grid(row=1, column=0)
        self.DictAllTagsSearch.grid(row=5, column=4, sticky=tk.N+tk.S, rowspan=9)
        self.SaveClearBox.grid(row=6, column=1)
        self.SaveWordButton.grid(row=0, column=0)
        self.ClearWordButton.grid(row=0, column=1)
        self.WordCraftBoxGrid()
        self.DefSearchListBox.grid(row=0, column=4, rowspan=4)
        return

    def SylGrid(self):
        # Syl Search List Box
        self.SylSearchListBox.grid(row=0, column=0, rowspan=4)
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

    def save_word_def(self, *events):
        word = self.DictChosenVar.get().replace('/', '')
        is_variant = self.word_variant(word)
        if is_variant != 'Variant':
            self.dictionary[word] = self.ChosenWordDef.get('0.0', tk.END)
        else:
            tk.messagebox.showinfo('Info', 'Word is a variant, cannot add to dictionary.')
        self.update_dictionary_list()
        return

    def delete_word(self, *events):
        word = self.DictChosenVar.get().replace('/', '')
        self.dictionary.pop(word)
        self.delete_word_from_tags(word)
        if word in self.taken:
            self.taken.remove(word)
            self.available.append(word)
        self.update_dictionary_list()
        return

    def delete_word_from_tags(self, word):
        for i in self.tags.keys():
            if word in self.tags[i]:
                self.tags[i].remove(word)
        return

    def word_selected(self, *events):
        word = self.ExistingWordSearchBox.get_curitem().replace('/', '')
        self.DictChosenVar.set(self.out(word))
        self.ChosenWordDef.delete('0.0', tk.END)
        if word in self.dictionary.keys():
            self.ChosenWordDef.insert(tk.END, self.lookup_def(word))
        self.ChosenWordTags.delete(0, tk.END)
        for i in self.get_tags(word):
            self.ChosenWordTags.insert(tk.END, i)
        return

    def add_word_tag(self, *events):
        tag = self.DictAllTagsSearch.get_curitem()
        word = self.DictChosenVar.get().replace('/', '')
        if word not in self.tags[tag]:
            self.tags[tag].append(word)
        self.update_word_tags(word)
        return

    def update_word_tags(self, word):
        self.ChosenWordTags.delete(0, tk.END)
        for i in self.get_tags(word):
            self.ChosenWordTags.insert(tk.END, i)
        return

    def delete_word_tag(self, *events):
        tag = self.ChosenWordTags.curitem()
        word = self.DictChosenVar.get().replace('/', '')
        if word in self.tags[tag]:
            self.tags[tag].remove(word)
        self.update_word_tags(word)
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
        val = self.SylSearchListBox.get_curitem()
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
        self.update_dictionary_list()
        return

    def update_dictionary_list(self):
        self.ExistingWordSearchBox.update_list([self.out(i) for i in self.dictionary.keys()])
        self.DefSearchListBox.update_list([self.out(i) for i in self.dictionary.keys()])
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
        self.update_dictionary_list()
        return

    def load(self):
        temp = pickle.load(open('data.p', 'rb'))
        self.reset_syllables()
        self.taken = temp['taken']
        self.available = temp['available']
        self.dictionary = temp['dictionary']
        self.tags = temp['tags']
        # TODO ensure all the important tags are there.
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

    def reset_syllables(self):
        self.syllables = [x + y for x in consonants for y in vowels]
        self.syllables.extend([y + x for x in consonants for y in vowels])
        self.syllables.extend(vowels)
        return

    def reset(self):
        self.reset_syllables()
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
        if tag not in self.tags.keys() and tag != '':
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
        if word in self.dictionary.keys():
            return 'Collision', ''
        return self.word_variant(word)

    def word_variant(self, word='ik-u-ri-ci-i'):
        # For each syllable, check if it's an affix then check if they are
        # arranged such that they would be another word's variant.
        syls = word.split('-')
        sylType = []
        tagOrder = []
        word_root = [-1, 100000]  # holds the indices of the root words.
        for syl in syls:
            sylTags = self.get_tags(syl)
            root = True
            # get the affix for the syllable
            for tag in sylTags:
                if 'Affix' in tag or 'Flag' in tag:
                    sylType.append(tag)
                    root = False
            # if no affix or flag, it's a root.
            if root:
                sylType.append('Root')
            # Get the position in affix order
            tagOrder.append(WordAffixOrder.index(sylType[-1]))
        # get the root syllables (cut away until it is found.
        word_root[0], word_root[1], lone_flag = self.get_root_bounds(tagOrder)
        # do checks for the end
        is_sorted = all(tagOrder[i] <= tagOrder[i+1] for i in range(len(tagOrder)-1))
        if is_sorted and not self.dup_affixes(tagOrder):
            if all([x == 7 for x in tagOrder]):
                ret = ''
            else:
                ret = 'Variant'
        else:
            ret = 'Improper Affix Order'
        if self.dup_affixes(tagOrder):
            ret = 'Duplicate Affixes'
        elif lone_flag:
            ret = 'Lone Flag'
        root_word = [x for x in syls if syls.index(x) >= word_root[0] and syls.index(x) <= word_root[1]]
        ret_word = ''
        for i in root_word:
            ret_word += i + '-'
        ret_word = ret_word[0:-1]
        # print(word)
        # print(tagOrder)
        # print(ret, word_root, root_word, ret_word)
        return ret, ret_word

    def get_root_bounds(self, tagOrder):
        word_root = [-1, 100000]
        # Looking for flags without appropriate affixes.
        lone_flag = False
        # going up
        seen = set()
        seen.add(-1)
        for tag, index in zip(tagOrder, range(len(tagOrder))):
            if tag in seen or tag >= 7:
                word_root[0] = index
            elif tag < max(seen):
                word_root[0] = index
            elif index+1 < len(tagOrder):
                if tagOrder[index] == 1 and tagOrder[index+1] != 2:
                    word_root[0] = index
                    lone_flag = True
            if word_root[0] > -1:
                break
            seen.add(tag)
        # going down
        seen = set()
        seen.add(100)
        for tag, index in zip(reversed(tagOrder), reversed(range(len(tagOrder)))):
            if tag in seen or tag <= 7:
                word_root[1] = index
                break
            elif tag > min(seen):
                word_root[1] = index
                break
            seen.add(tag)
        return word_root[0], word_root[1], lone_flag

    def dup_affixes(self, tagOrder):
        rootord = WordAffixOrder.index('Root')
        seen = set()
        for i in tagOrder:
            if i == rootord:
                continue
            elif i in seen:
                return True
            seen.add(i)
        return False


if __name__ == '__main__':
    # load from pickle
    curr = DictionaryApp(True)
    curr.mainloop()
