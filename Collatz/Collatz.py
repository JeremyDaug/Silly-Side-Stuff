"""
Collatz conjecture calculator window.
"""

import time


class Collatz:
    def __init__(self):
        self.num = 1
        self.Done = 0
        self.tree = {1: [2]}

    def fill_tree_to_step(self, step):
        i = 1
        while i <= step:
            current_step = self.get_numbers_with_steps(i)
            for j in current_step:
                if self.__is_three_plus_one(j):
                    self.tree[j] = [self.collatz_step(j),
                                    self.reverse_half(j),
                                    self.reverse_triple_plus_one(j)]
                else:
                    self.tree[j] = [self.collatz_step(j),
                                    self.reverse_half(j)]
            if i == step:
                return
            i += 1

    def get_numbers_with_steps(self, steps):
        current_step = [1]
        i = 1
        while i <= steps:
            next_step = []
            for j in current_step:
                next_step.append(self.reverse_half(j))
                if self.reverse_triple_plus_one(j) and not j == 4:
                    next_step.append(self.reverse_triple_plus_one(j))
            current_step = next_step
            if i == steps:
                return current_step
            i += 1

    def next_odd(self, number=None):
        if number is None:
            number = self.num
        ret = number
        while self.__is_even(ret):
            ret = self.collatz_step(ret)
        return ret

    def steps_to_odd(self, number=None):
        if number is None:
            number = self.num
        temp = number
        ret = 0
        while self.__is_even(temp):
            ret += 1
            temp = self.collatz_step(temp)
        return ret

    def steps_to_1(self, number=None):
        if number is None:
            number = self.num
        ret = 1
        temp = number
        while not temp == 1:
            ret += 1
            temp = self.collatz_step(temp)
        return ret

    def collatz_step(self, number=None):
        if number is None:
            number = self.num
        if self.__is_even(number):
            number /= 2
        else:
            number = 3 * number + 1
        return int(number)

    def collatz_list(self, number=None):
        if number is None:
            number = self.num
        ret = [number]
        while not ret[-1] == 1:
            ret.append(int(self.collatz_step(ret[-1])))
        return ret

    def __is_even(self, number=None):
        if number is None:
            number = self.num
        if (number % 2) == 0:
            return True
        else:
            return False

    def __is_three_plus_one(self, number=None):
        if number is None:
            number = self.num
        if self.__is_whole((number-1)/3):
            return True
        else:
            return False

    def __is_whole(self, number=None):
        if number is None:
            number = self.num
        if (number % 1) == 0:
            return True
        else:
            return False

    def adjacent_numbers(self, number=None):
        if number is None:
            number = self.num
        ret = {'double': 2*number}
        if self.__is_even(number):
            ret['half'] = int(number/2)
        else:
            ret['half'] = None
        if self.__is_three_plus_one(number):
            ret['tpo'] = int((number-1)/3)
        else:
            ret['tpo'] = None
        return ret

    def take_next_step(self):
        self.num = self.collatz_step()
        return

    def reverse_half(self, number=None):
        if number is not None:
            return number*2
        else:
            self.num *= 2

    def reverse_triple_plus_one(self, number=None):
        if number is not None:
            if self.__is_three_plus_one(number):
                return int((number-1)/3)
            else:
                return False
        temp = (self.num-1)/3
        if self.__is_whole(temp):
            self.num = temp
            return True
        else:
            return False

    def is_prime(self, number=None):
        if number is None:
            number = self.num
        for i in range(2, int(number/2)):
            if number % i == 0:
                return False
        return True

    def next_prime(self, number=None):
        if number is None:
            number = self.num
        number = int(self.collatz_step(number))
        while not self.is_prime(number):
            number = int(self.collatz_step(number))
        return number

    def prime_factorization(self, number=None):
        if number is None:
            number = self.num
        ret = [number]
        i = 2
        while i < ret[-1]:
            if ret[-1] % i == 0 and self.is_prime(i):
                temp = ret[-1]
                ret[-1] = i
                ret.append(int(temp/i))
            else:
                i += 1
        return ret

    def factors(self, number=None):
        if number is None:
            number = self.num
        ret = [1]
        for i in range(2, number+1):
            if number % i == 0:
                ret.append(i)
        return ret


Coll = Collatz()


def print_all():
    print('Number: %d' % Coll.num)
    print('In Binary: %s' % bin(Coll.num))
    print('Next Number: %d' % Coll.collatz_step())
    print('Steps to 1: %d' % Coll.steps_to_1())
    print('Adjacent Numbers: ', Coll.adjacent_numbers())
    print('Next Odd Number: %d' % Coll.next_odd())
    print('Steps to this odd number: %d' % Coll.steps_to_odd())
    if Coll.is_prime():
        print('Is Prime Number: True')
    else:
        print('Is Prime Number: False')

if __name__ == '__main__':
    Done = False
    prompt = 'Input Starting Number\n'
    while not Done:
        text = input(prompt)
        text.strip()
        if prompt == 'Input Starting Number\n':
            Coll.num = int(text)
            prompt = 'What do you want to do now?\n'
            print_all()
        elif prompt == 'What do you want to do now?\n':
            if str(text).lower() == 'help'.lower():
                print('valid inputs (case insensitive):\n'
                      'help: Get more info on valid inputs.\n'
                      'take step: Move the number to the next step.\n'
                      'print list: Get the full list of collatz numbers down to 1.\n'
                      'vertical list: Get the List, but print it vertically.'
                      'Change Number = ##: Change the number currently looked at.\n'
                      'Double: Take a step up the even route and double the number.\n'
                      'Reverse tpo: Take a step up the 3x+1 route if available.\n'
                      'next prime: Find the Next prime number down the line.\n'
                      'prime factorization: Get the Prime Factorization of the number.\n'
                      'numbers with distance: Get the numbers so many steps away.\n'
                      'done/quit: Exit the program.')
            elif str(text).lower() == 'print list'.lower():
                print(Coll.collatz_list())
            elif str(text).lower() == 'Done'.lower() or str(text).lower() == 'quit'.lower():
                Done = True
                print('Goodbye.')
            elif str(text).lower() == 'Take Step'.lower():
                Coll.take_next_step()
                print_all()
            elif 'Change Number'.lower() in str(text).lower():
                Coll.num = int(input('Input New Number \n'))
                print_all()
            elif str(text).lower() == 'double'.lower():
                Coll.reverse_half()
                print_all()
            elif str(text).lower() == 'Reverse tpo'.lower():
                if Coll.reverse_triple_plus_one():
                    print_all()
                else:
                    print('Invalid action for this number.')
            elif str(text).lower() == 'print Binary List'.lower():
                temp = Coll.collatz_list()
                for i in temp:
                    print('{:7d} : {:>20}'.format(int(i), bin(i)[2:]))
                    time.sleep(0.5)
            elif str(text).lower() == 'vertical List'.lower():
                for i in Coll.collatz_list():
                    print(i)
            elif str(text).lower() == 'next prime'.lower():
                print(Coll.next_prime())
            elif str(text).lower() == 'prime list'.lower():
                temp = Coll.collatz_list()
                for i in temp:
                    print('{:10d} : {}'.format(int(i), Coll.prime_factorization(i)))
            elif str(text).lower() == 'prime factorization'.lower():
                print(Coll.prime_factorization())
            elif str(text).lower() == 'factors'.lower():
                print(Coll.factors())
            elif str(text).lower() == 'numbers with distance'.lower():
                print(Coll.get_numbers_with_steps(int(input('How Many Steps:\n'))))
            elif str(text).lower() == 'fill and print tree'.lower():
                Coll.fill_tree_to_step(int(input('How many steps should be filled?\n')))
                for i in Coll.tree.keys():
                    print(i, ':', Coll.tree[i])
            else:
                print('Invalid input, if you need help type in help to get valid inputs.')
