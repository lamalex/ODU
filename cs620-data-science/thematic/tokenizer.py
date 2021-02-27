#!/usr/bin/env python3
"""
Perform hierarchal agglomerative clustering on tokenized
and vectorized movie scripts to find families of similar
films based on thematic content.

CS620
Semester project
@author: Alex Launi
"""
import re
import string
from typing import List
import nltk
from nltk.stem.snowball import SnowballStemmer

_RE_WHITESPACE = re.compile(r'\s+', re.MULTILINE | re.IGNORECASE)
_RE_METADATA = re.compile(r'Writers.*', re.IGNORECASE)


def tokenize(text: str) -> List[str]:
    """
    Take unprocessed script input and clean the data for follow on
    processing.
      - Trim newlines and excess spacing that is present in scripts
      - Truncate trailing imsdb metadata
      - Tokenize
      - Remove punctuation marks
      - Convert to lower case
    """
    # Process input in its string format
    cleaned = _process_as_string(text)
    # tokenize
    cleaned = nltk.word_tokenize(cleaned)
    # process after tokenization
    tokens = _process_tokenized(cleaned)
    return tokens


def _process_as_string(text: str) -> str:
    cleaned = _remove_punctuation(text)
    cleaned = _compress_whitespace(cleaned)
    cleaned = _remove_trailing_metadata(cleaned)
    return cleaned


def _remove_punctuation(text: str) -> str:
    punctuation_replacement_table = text.maketrans(
        '', '', string.punctuation)
    return text.translate(punctuation_replacement_table)


def _compress_whitespace(text: str) -> str:
    return re.sub(_RE_WHITESPACE, ' ', text)


def _remove_trailing_metadata(text: str) -> str:
    return re.sub(_RE_METADATA, '', text)


def _process_tokenized(tokens: List[str]) -> List[str]:
    stemmer = SnowballStemmer('english', ignore_stopwords=True)
    clean_tokens = _normalize_case(tokens)
    return [stemmer.stem(token) for token in clean_tokens]


def _normalize_case(text: List[str]) -> List[str]:
    return [w.lower() for w in text]
