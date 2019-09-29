"""
A side class for holding a listbox with scrollbar (currently only vertical)
"""
import tkinter as tk


class Scrollbox:
    def __init__(self, parent=None, contains=[]):
        if parent is None:  # If no parent crash.
            raise RuntimeError('ScrollList must have a parent.')
        # make the frame
        self.Frame = tk.Frame(parent)
        # get the listbox and scrollbar set
        self.Scrollbar = tk.Scrollbar(self.Frame, orient=tk.VERTICAL)
        self.List = tk.Listbox(self.Frame, yscrollcommand=self.Scrollbar.set,
                               exportselection=False)
        self.Scrollbar.config(command=self.List.yview)
        # set the Listbox and Scrollbar
        # add anything from contains to the listbox
        if contains:
            for i in contains:
                self.List.insert(tk.END, i)

        return

    def size(self):
        return self.List.size()

    def bind_listbox(self, flag, callback):
        self.List.bind(flag, callback)
        return

    def bind_scrollbar(self, sequence=None, func=None, add=None):
        self.Scrollbar.bind(sequence, func, add)
        return

    def pack(self, conf={}, **kwargs):
        self.Frame.pack(conf, **kwargs)
        return

    def grid(self, row=0, column=0, sticky='', rowspan=1, columnspan=1):
        self.Frame.grid(row=row, column=column, sticky=sticky, rowspan=rowspan, columnspan=columnspan)
        self.List.pack(side='left', expand=True, fill=tk.BOTH)
        self.Scrollbar.pack(side='right', fill=tk.Y)
        return

    def insert(self, index, elements):
        self.List.insert(index, elements)
        return

    def delete(self, first, last):
        self.List.delete(first, last)
        return

    def curselection(self):
        return self.List.curselection()

    def curitem(self):
        return self.List.get(self.List.curselection())

    def get(self, first, last=None):
        return self.List.get(first, last)
