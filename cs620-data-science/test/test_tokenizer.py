import thematic.tokenizer
import pytest

def test_remove_whitespace():
    given = "This  string\t has \n lot's of \n \n \n white\t\rspace"
    expected = "This string has lot's of white space"
    assert thematic.tokenizer._compress_whitespace(given) == expected

def test_clean_metadata():
    given = "Writers: Amanda Huginkiss back to imsdb.com"
    expected = ""
    assert thematic.tokenizer._remove_trailing_metadata(given) == expected