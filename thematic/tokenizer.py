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
    cleaned = _process_tokenized(cleaned)

    return cleaned


def _process_as_string(text: str) -> str:
    cleaned = _compress_whitespace(text)
    cleaned = _remove_trailing_metadata(cleaned)
    return cleaned


def _compress_whitespace(text: str) -> str:
    return re.sub(_RE_WHITESPACE, ' ', text)


def _remove_trailing_metadata(text: str) -> str:
    return re.sub(_RE_METADATA, '', text)


def _process_tokenized(text: List[str]) -> List[str]:
    cleaned = _remove_punctuation(text)
    cleaned = _normalize_case(cleaned)
    return cleaned


def _remove_punctuation(text: List[str]) -> List[str]:
    punctuation_replacement_table = text.maketrans(
        '', '', string.punctuation).j
    return [w.translate(punctuation_replacement_table)
            for w in text if w not in string.punctuation]


def _normalize_case(text: List[str]) -> List[str]:
    return [w.lower() for w in text]
