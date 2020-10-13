#!/usr/bin/env python3
"""
Perform hierarchal agglomerative clustering on scripts downloaded using
download_data.py. Output tree structure for processing with d3.js to
clusters.json

hyperparameter defaults:
  - ngram_shape: (5, 10)
  - linkage: weighted
"""

import os
import json
import logging
from functools import reduce
from scipy.spatial import distance
from scipy.cluster import hierarchy
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import cosine_distances
from tokenizer import tokenize
from utils import setup_logging, DATA_DIR


def _file_name_to_movie_title(file: str) -> str:
    """
    Clean up file name to remove ' Script.txt'
    """
    raw_from_disk = os.path.splitext(os.path.basename(file))[0]
    return (raw_from_disk[:-len('Script')]
            if raw_from_disk.endswith("Script") else raw_from_disk).strip()


def _add_node(node, parent):
    """
    Create a nested dictionary from the ClusterNode's returned by SciPy
    """
    # First create the new node and append it to its parent's children
    new_node = dict(node_id=node.id, children=[])
    parent["children"].append(new_node)
    # Recursively add the current node's children
    if node.left:
        _add_node(node.left, new_node)
    if node.right:
        _add_node(node.right, new_node)


def _label_tree(node, name_map):
    """
    Label each node with the names of each leaf in its subtree
    """
    # If the node is a leaf, then we have its name
    if len(node["children"]) == 0:
        leaf_names = [name_map[node["node_id"]]]

    # If not, flatten all the leaves in the node's subtree
    else:
        leaf_names = [
            reduce(
                lambda ls,
                c: ls +
                _label_tree(
                    c,
                    name_map),
                node["children"],
                [])]

    # Delete the node id since we don't need it anymore and
    # it makes for cleaner JSON
    del node["node_id"]

    # Labeling convention: "-"-separated leaf names
    node["name"] = "-".join(sorted(map(str, leaf_names)))
    return leaf_names


def cluster():
    """
    Read all script files and vectorize them using TF-IDF vectorizer.
    Use cosine distance to perform hierarchal clustering on ngrams
    """
    ngram_shape = (5, 10)

    scripts = {}
    script_files = map(
        lambda f: os.path.join(
            DATA_DIR,
            f),
        os.listdir(DATA_DIR))

    for script_file in script_files:
        if not script_file.endswith(".txt"):
            continue

        with open(script_file) as reader:
            movie_title = _file_name_to_movie_title(script_file)
            raw_script = reader.read()
            scripts[movie_title] = raw_script
            logging.info("Read in %s (%d)", movie_title, len(raw_script))

    logging.info("Begin Vectorizing")
    tfidf_vectorizer = TfidfVectorizer(
        use_idf=True,
        lowercase=False,
        tokenizer=tokenize,
        ngram_range=ngram_shape)
    tfidf_matrix = tfidf_vectorizer.fit_transform(scripts.values())
    logging.info("End Vectorizing")

    logging.info("Begin Clustering")
    cosine_dist = cosine_distances(tfidf_matrix)
    cosine_dist_matrix = distance.pdist(cosine_dist)
    clusters = hierarchy.linkage(cosine_dist_matrix, method="weighted")
    tree = hierarchy.to_tree(clusters)
    logging.info("End Clustering")

    id_name_map = dict(zip(range(len(scripts.keys())), scripts.keys()))
    d3_dendro = dict(children=[], name="Root1")
    _add_node(tree, d3_dendro)
    _label_tree(d3_dendro["children"][0], id_name_map)

    # Output to JSON for D3
    json.dump(d3_dendro, open("clusters.json", "w"), sort_keys=True, indent=2)
    logging.info("Finished!")

if __name__ == "__main__":
    setup_logging()
    cluster()
