#!/usr/bin/env python3

import os
import json
import fileinput
import logging
from functools import reduce
from scipy.spatial import distance
from scipy.cluster import hierarchy
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import cosine_distances
from tokenizer import tokenize
from utils import setup_logging, DATA_DIR

def file_name_to_movie_title(file: str) -> str:
    return os.path.splitext(os.path.basename(file))[0]

def cluster():
    # load all scripts into `movie title: script` dictionary
    scripts = {}
    script_files = map(lambda f: os.path.join(DATA_DIR, f), os.listdir(DATA_DIR))
    for script_file in script_files:
        with open(script_file) as reader:
            movie_title = file_name_to_movie_title(script_file)
            raw_script = reader.read()
            scripts[movie_title] = raw_script
            logging.info("Read in %s (%d)", movie_title, len(raw_script))

    logging.info("Vectorizing")
    tfidf_vectorizer = TfidfVectorizer(use_idf=True, lowercase=False, tokenizer=tokenize, ngram_range=(1,3))
    tfidf_matrix = tfidf_vectorizer.fit_transform(scripts.values())
    cosine_dist = cosine_distances(tfidf_matrix)

    cosine_dist_matrix = distance.pdist(cosine_dist)
    clusters = hierarchy.linkage(cosine_dist_matrix, method="weighted")
    tree = hierarchy.to_tree(clusters)
    
    id2name = dict(zip(range(len(scripts.keys())), scripts.keys()))
    
    # Create a nested dictionary from the ClusterNode's returned by SciPy
    def add_node(node, parent ):
        # First create the new node and append it to its parent's children
        newNode = dict( node_id=node.id, children=[] )
        parent["children"].append( newNode )
        # Recursively add the current node's children
        if node.left: add_node( node.left, newNode )
        if node.right: add_node( node.right, newNode )
        
    # Initialize nested dictionary for d3, then recursively iterate through tree
    d3Dendro = dict(children=[], name="Root1")
    add_node(tree, d3Dendro )

    # Label each node with the names of each leaf in its subtree
    def label_tree( n ):
        # If the node is a leaf, then we have its name
        if len(n["children"]) == 0:
            leafNames = [ id2name[n["node_id"]] ]
        
        # If not, flatten all the leaves in the node's subtree
        else:
            leafNames =[ reduce(lambda ls, c: ls + label_tree(c), n["children"], [])]

        # Delete the node id since we don't need it anymore and
        # it makes for cleaner JSON
        del n["node_id"]

        # Labeling convention: "-"-separated leaf names
        n["name"] = "-".join(sorted(map(str, leafNames)))
        return leafNames

    label_tree( d3Dendro["children"][0] )

    # Output to JSON for D3
    json.dump(d3Dendro, open("clusters.json", "w"), sort_keys=True, indent=4)

if __name__ == "__main__":
    setup_logging()
    cluster()
