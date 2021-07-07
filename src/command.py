"""
Command Pattern
===============

The command pattern in Python could be realized simply with first class functions.

To make the usage more interesting, I'll implement undoable commands:
- The `TextEditor` class holds a text buffer and a history of commands.
  This class knows nothing about how to manipulate a text buffer; it simply
  invokes commands and remembers which commands have been invoked in order.
- The different commands know how to manipulate the text buffer and how to
  undo their actions.
"""
from abc import ABC, abstractmethod


class TextEditor:
    def __init__(self, inital_text=''):
        self.text = TextBuffer(inital_text)
        self.command_history = []

    def insert(self, at, text):
        cmd = InsertCommand(self.text, at, text)
        self.execute_command(cmd)

    def delete(self, at, len):
        cmd = DeleteCommand(self.text, at, len)
        self.execute_command(cmd)

    def append(self, text):
        self.insert(len(self.text), text)

    def undo(self):
        try:
            last_cmd = self.command_history.pop()
        except IndexError:
            return
        last_cmd.undo()

    def execute_command(self, cmd):
        cmd.execute()
        self.command_history.append(cmd)


class Command(ABC):
    @abstractmethod
    def execute(self): pass

    @abstractmethod
    def undo(self): pass


class InsertCommand(Command):
    def __init__(self, text_buffer, at, text):
        self.text_buffer = text_buffer
        self.position = at
        self.text = text

    def execute(self):
        self.text_buffer.insert(self.position, self.text)

    def undo(self):
        self.text_buffer.delete_range(self.position, len(self.text))


class DeleteCommand(Command):
    def __init__(self, text_buffer, at, length):
        self.text_buffer = text_buffer
        self.position = at
        self.length = length
        self.deletion = None

    def execute(self):
        self.deletion = self.text_buffer.delete_range(self.position, self.length)

    def undo(self):
        assert self.deletion is not None
        self.text_buffer.insert(self.position, self.deletion)


class TextBuffer:
    def __init__(self, text):
        self.text = text

    def insert(self, at, text):
        self.text = f'{self.text[:at]}{text}{self.text[at:]}'

    def delete_range(self, start, length):
        deleted = self.text[start:start + length]
        self.text = f'{self.text[:start]}{self.text[start + length:]}'
        return deleted

    def __len__(self):
        return len(self.text)

    def __eq__(self, other):
        if isinstance(other, TextBuffer):
            return self.text == other.text
        else:
            return self.text == other


def test_insert_text():
    ed = TextEditor('')
    ed.insert(0, 'hello')
    assert ed.text == 'hello'


def test_insert_text_before():
    ed = TextEditor('world')
    ed.insert(0, 'hello ')
    assert ed.text == 'hello world'


def test_insert_text_middle():
    ed = TextEditor('helloworld')
    ed.insert(5, ', ')
    assert ed.text == 'hello, world'


def test_undo_nothing():
    ed = TextEditor('hello')
    ed.undo()
    assert ed.text == 'hello'


def test_undo():
    ed = TextEditor('foo')
    ed.insert(3, 'bar')
    assert ed.text == 'foobar'
    ed.undo()
    assert ed.text == 'foo'


def test_delete():
    ed = TextEditor('hexxxxllo')
    ed.delete(2, 4)
    assert ed.text == 'hello'


def test_undo_delete():
    ed = TextEditor('hello')
    ed.delete(4, 1)
    assert ed.text == 'hell'
    ed.undo()
    assert ed.text == 'hello'


def test_multi_undo():
    ed = TextEditor('')
    ed.append('hello')
    assert ed.text == 'hello'
    ed.append(', earth!')
    assert ed.text == 'hello, earth!'
    ed.undo()
    assert ed.text == 'hello'
    ed.append(', world!')
    assert ed.text == 'hello, world!'
    ed.undo()
    assert ed.text == 'hello'
    ed.undo()
    assert ed.text == ''
