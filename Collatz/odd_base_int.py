"""
A class for oddly based classes.
"""


class IntBase:
    def __init__(self, num, base, min_digits=1):
        self.__check_type(num)
        self.__check_type(base)
        if base < 2:
            raise ValueError('IntBase must have a base higher than 1.')
        self.__val = num
        self.__base = base
        self.__min_digits = min_digits

    def set_value(self, num):
        self.__check_type(num)
        self.__val = num
        return

    def set_base(self, num):
        self.__check_type(num)
        self.__base = num
        return

    def get_num(self):
        temp = self.__val
        ret = ''
        while temp != 0:
            end = temp % self.__base
            temp -= end
            ret = end + ret
            temp /= self.__base
        if len(ret) < self.__min_digits:
            newtemp = len(ret)
            for i in range(self.__min_digits-newtemp):
                ret += 0 + ret
        return ret

    def set_min_digits(self, num):
        self.__check_type(num)
        if num < 1:
            num = 1
        self.__min_digits = num

    def __str__(self):
        return self.get_num()

    def __repr__(self):
        return self.get_num()

    @staticmethod
    def __check_type(var):
        if not isinstance(var, int):
            raise TypeError('IntBase only takes ints. Either double check or cast the number to int')
