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
