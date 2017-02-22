"""
A helper to get a scrolllist, label, and search functionality nice and quickly.
"""

import tkinter as tk
from Scrollbox import Scrollbox


class SearchListBox:
    def __init__(self, parent=None, label='', parent_list=[], search_func=None):
        """
        Init function
        :param tk.Frame parent: The parent frame.
        :param str label: The name of the sreachbox
        :param list(str) parent_list: A link to a list of things we'll be searching consistently,
            being callable would be smart.
        :param func search_func: A function call that will be searched by, if none given, then name is assumed.
        """
        if not parent:
            raise RuntimeError('SearchListBox must have a parent.')

        self.myframe = tk.Frame(parent)
        self.mylabel = tk.Label(self.myframe, text=label)
        self.mylist = Scrollbox(self.myframe, parent_list)
        self.list_store = parent_list
        self.mysearchvar = tk.StringVar()
        if search_func:
            self.mysearchvar.trace_variable('w', search_func)
        else:
            self.mysearchvar.trace_variable('w', self.defaultsearch)
        self.mysearch = tk.Entry(self.myframe, textvariable=self.mysearchvar)
        self.set_grid()
        return

    def set_grid(self):
        self.mylabel.grid(row=0, column=0)
        self.mylist.grid(row=1, column=0)
        self.mysearch.grid(row=2, column=0)
        return

    def bind_listbox(self, flag, func):
        self.mylist.bind_listbox(flag, func)
        return

    def get_search_var(self):
        return self.mysearchvar.get()

    def get_curitem(self):
        return self.mylist.curitem()

    def bind_scrollbar(self, conf={}, **kwargs):
        self.mylist.bind_scroolbar(conf, **kwargs)
        return

    def list_size(self):
        return self.mylist.size()

    def pack(self, conf={}, **kwargs):
        self.myframe.pack(conf, **kwargs)
        return

    def grid(self, row=0, column=0, sticky='', rowspan=1, columnspan=1):
        self.myframe.grid(row=row, column=column, sticky=sticky, rowspan=1, columnspan=1)
        return

    def defaultsearch(self, *events):
        search = self.mysearchvar.get()
        self.mylist.delete(0, tk.END)
        if not search:
            for i in self.list_store:
                self.mylist.insert(tk.END, i)
        else:
            for i in self.list_store:
                if search in i:
                    self.mylist.insert(tk.END, i)
        return

    def update_list(self, new_list):
        self.list_store = new_list
        self.mysearchvar.set('')
