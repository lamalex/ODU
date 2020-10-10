#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Mon Jul 22 22:49:30 2019

@author: alexlauni
"""
import os
import numpy as np
import pandas as pd
import sqlalchemy as db
from scipy import interp
import seaborn as sns
import matplotlib.pyplot as plt

class OULAD:
    def __init__(self):
        db_user = os.getenv('OULAD_DB_USER')
        db_pass = os.getenv('OULAD_DB_PASSWORD')
        db_connect_uri = "postgresql://{}:{}@localhost:5432/cs773".format(db_user, db_pass)

        self.engine = db.create_engine(db_connect_uri)
        self.conn = self.engine.connect()
        self.metadata = db.MetaData()

    def __del__(self):
        self.conn.close()

    def query_table(self, table_name):
        table = db.Table(table_name, self.metadata, autoload=True, autoload_with=self.engine)
        query = db.select([table])
        ResultProxy = self.conn.execute(query)
        ResultSet = ResultProxy.fetchall()
        ResultProxy.close()
        df = pd.DataFrame(ResultSet)
        df.columns = ResultSet[0].keys()
        return df

def remove_withdraws(df):
    return df[df['final_result'] != 'Withdrawn']

def set_risk_class(df):
        df['class'] = df['final_result'].apply(lambda x: 'at risk' if x in ['Fail', 'Withdrawn'] else 'not at risk')
        return df

def group_by_feature_and_risk(df, feature):
    grouped = df.groupby([feature, 'class'])['id_student'].count() # id_student just used to minimize dataframe shape
    grouped.rename(columns={'id_student': 'count'}, inplace=True)
    grouped = grouped.reset_index().rename(columns={0: 'count'})
    return grouped

def normalize_by_feature_and_risk(df, feature):
    grouped = df.groupby([feature, 'class'])['id_student'].count() # id_student just used to minimize dataframe shape
    grouped.rename(columns={'id_student': 'count'}, inplace=True)
    normalized = grouped.groupby(level=0).apply(lambda x: x / float(x.sum()))
    normalized = normalized.reset_index().rename(columns={0: 'pct'})
    return normalized

def generate_combined_risk_feature_plot(df, feature, xlabel_rotation=0):
    risk_feature_count = group_by_feature_and_risk(studentinfo_df, feature)
    risk_feature_pcts = normalize_by_feature_and_risk(studentinfo_df, feature)
    datasets = [risk_feature_count, risk_feature_pcts]
    fig, ax = plt.subplots(1, 2)
    fig.suptitle("Risk by {} (withdrawn removed)".format(feature))
    for dataset, subplot in zip(datasets, ax.flatten()):
        sns.catplot(kind='bar', data=dataset, x=feature, hue='class', y=dataset.columns[-1], ax=subplot, hue_order=['not at risk', 'at risk'])
    ax[0].set_title("Count")
    ax[1].set_title("Normalized")
    ax[1].set_ylim((0,1))
    for subplot in ax.flatten():
        for label in subplot.get_xticklabels():
            label.set_rotation(xlabel_rotation)
    return (fig, ax)

oulad = OULAD()

studentinfo_df = oulad.query_table("studentinfo")
set_risk_class(studentinfo_df)
studentinfo_df_no_wd = remove_withdraws(studentinfo_df)

# Normalize gender to see percent of total GENDER at risk/not at risk
(fig, ax) = generate_combined_risk_feature_plot(studentinfo_df, 'gender')

# Normalize regions to see percent of total FROM REGION at risk/not at risk
(fig, ax) = generate_combined_risk_feature_plot(studentinfo_df, 'region', xlabel_rotation=90)

# Normalize highest_education to see percent of total HIGHEST ED at risk/not at risk
(fig, ax) = generate_combined_risk_feature_plot(studentinfo_df, 'highest_education', xlabel_rotation=90)

# Normalize age_band to see percent of total AGE BAND at risk/not at risk
(fig, ax) = generate_combined_risk_feature_plot(studentinfo_df, 'age_band')

# Normalize disability to see percent of total AGE BAND at risk/not at risk
(fig, ax) = generate_combined_risk_feature_plot(studentinfo_df, 'disability')

# Normalize term to see percent of total code_presentation at risk/not at risk
# Didnt want to bother figuring out this abstraction. whatever.
feature = studentinfo_df['code_presentation'].str[-1]
risk_term_counts = group_by_feature_and_risk(studentinfo_df, feature)
risk_term_pcts = normalize_by_feature_and_risk(studentinfo_df, feature)
fig, ax = plt.subplots(1, 2)
fig.suptitle("Risk by {} (withdrawn removed)".format("term"))
for dataset, subplot in zip([risk_term_counts, risk_term_pcts], ax.flatten()):
    dataset.rename(columns={'code_presentation': 'term'}, inplace=True)
    sns.catplot(kind='bar', data=dataset, x='term', hue='class', y=dataset.columns[-1], ax=subplot, hue_order=['not at risk', 'at risk'])
ax[0].set_title("Count")
ax[1].set_title("Normalized")
ax[1].set_ylim((0,1))


numeric_features = ['num_of_prev_attempts', 'studied_credits']
nominal_features = ['code_module', 'code_presentation', 'gender', 'region', 'highest_education', 'imd_band', 'age_band', 'disability', 'final_result', 'class']

studentvle_df = oulad.query_table("studentvle")
studentvle_df_with_risk = studentvle_df.join(studentinfo_df[['id_student', 'final_result']].set_index('id_student'), on='id_student')
studentvle_df_with_risk = set_risk_class(studentvle_df_with_risk)

assessments_df = oulad.query_table("assessments")
studentassessments_df = oulad.query_table("studentassessment")

studentassessments_with_duedate_df = studentassessments_df.join(assessments_df[['id_assessment', 'date']].set_index('id_assessment'), on='id_assessment')
studentassessments_with_duedate_df = studentassessments_with_duedate_df.join(studentinfo_df[['id_student', 'class', 'final_result']].set_index('id_student'), on='id_student')
studentassessments_with_duedate_df['days after'] = studentassessments_with_duedate_df['date_submitted'] - studentassessments_with_duedate_df['date']
studentassessments_with_duedate_df = remove_withdraws(studentassessments_with_duedate_df)

# Drop withdraws
studentvle_df_with_risk_no_wd = remove_withdraws(studentvle_df_with_risk)


###
# EDA
###

# Seaborn setup
sns.set(style='whitegrid', palette="deep", font_scale=1.1, rc={"figure.figsize": [8, 5]})

## DATA WITH WITHDRAWN => AT RISK
fig, ax = plt.subplots(1, 2)
fig.suptitle("Numeric feature histograms (withdraws included)")
for feature, subplot in zip(numeric_features, ax.flatten()):
    sns.distplot(
            studentinfo_df[feature], norm_hist=False, kde=False, hist_kws={"alpha": 1}, ax=subplot
    ).set(xlabel=feature, ylabel="Count")

# All nominal count bargraphs
fig, ax = plt.subplots(2, 5, figsize=(20, 10))
fig.suptitle("Nominal Features (withdrawn included)")
for feature, subplot in zip(nominal_features, ax.flatten()):
    sns.countplot(studentinfo_df[feature], ax=subplot)
    for label in subplot.get_xticklabels():
        label.set_rotation(45)

# All nominal catplots
fig, ax = plt.subplots(2, 4, figsize=(10, 10))
fig.suptitle("Nominal Features x Class (withdrawn included)")
for feature, subplot in zip(nominal_features, ax.flatten()):
    sns.catplot(x=feature, kind="count", hue="class", data=studentinfo_df[:-2], ax=subplot);
    for label in subplot.get_xticklabels():
        label.set_rotation(45)

# Box and whisker plots
fig, ax = plt.subplots(1, 2, figsize=(15, 10))
fig.suptitle("Numeric Features x Class (withdrawn included)")
for var, subplot in zip(numeric_features, ax.flatten()):
    sns.boxplot(x='class', y=var, data=studentinfo_df, ax=subplot)

# Clicks per day drawn as line plot
sns.lineplot(data=studentvle_df_with_risk, y='sum_click', x ='date', hue='class')

# Assessment days late drawn as line plot
g = sns.lineplot(data=studentassessments_with_duedate_df, x='date', y='days after', hue='class')
g.set_title("Lateness of Assignments (withdraws included)")

g = sns.catplot(kind='bar', data=risk_gender_pcts, x='gender', hue='class', y='pct')
g.set(ylim=(0,1))
g.fig.suptitle("Risk by Gender (normalized, withdraws included)")

g = sns.catplot(kind='bar', data=risk_region_pcts, x='region', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(ylim=(0,1))
g.set_xticklabels(rotation=90)
g.fig.suptitle("Risk by Region (normalized, withdraws included)")

g = sns.catplot(kind='bar', data=risk_ed_pcts, x='highest_education', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(ylim=(0,1))
g.set_xticklabels(rotation=90)
g.fig.suptitle("Risk by Highest Education (normalized, withdraws included)")

g = sns.catplot(kind='bar', data=risk_age_pcts, x='age_band', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(ylim=(0,1))
g.fig.suptitle("Risk by Age Band (normalized, withdraws included)")

g = sns.catplot(kind='bar', data=risk_disb_pcts, x='disability', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(ylim=(0,1))
g.fig.suptitle("Risk by Disability Status (normalized, withdraws included)")

g = sns.catplot(kind='bar', data=risk_term_pcts, x='code_presentation', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(xlabel="Term")
g.set(ylim=(0,1))
g.fig.suptitle("Risk by Semester Term (normalized, withdraws included)")


## DATA WITHOUT WITHDRAWN

# numeric features histograms
fig, ax = plt.subplots(1, 2)
fig.suptitle("Numeric feature histograms (withdraws removed)")
for feature, subplot in zip(numeric_features, ax.flatten()):
    sns.distplot(
            studentinfo_no_withdraws_df[feature], norm_hist=False, kde=False, hist_kws={"alpha": 1}, ax=subplot
    ).set(xlabel=feature, ylabel="Count")

# All nominal count bargraphs
fig, ax = plt.subplots(2, 5, figsize=(20, 10))
fig.suptitle("Nominal Features (withdrawn removed)")
for feature, subplot in zip(nominal_features, ax.flatten()):
    sns.countplot(studentinfo_no_withdraws_df[feature], ax=subplot)
    for label in subplot.get_xticklabels():
        label.set_rotation(45)

# All nominal catplots
fig, ax = plt.subplots(2, 4, figsize=(10, 10))
fig.suptitle("Nominal Features x Class (Withdraws removed)")
for feature, subplot in zip(nominal_features, ax.flatten()):
    sns.catplot(x=feature, kind="count", hue="class", data=studentinfo_no_withdraws_df[:-2], ax=subplot);
    for label in subplot.get_xticklabels():
        label.set_rotation(45)
c
# Box and whisker plots
fig, ax = plt.subplots(1, 2, figsize=(15, 10))
fig.suptitle("Numeric features x Risk (withdrawn removed)")
for var, subplot in zip(numeric_features, ax.flatten()):
    sns.boxplot(x='class', y=var, data=studentinfo_no_withdraws_df, ax=subplot)

# Clicks per day drawn as line plot
sns.lineplot(data=studentvle_df_with_risk_no_wd, y='sum_click', x ='date', hue='class', hue_order=['not at risk', 'at risk'])

g = sns.catplot(kind='bar', data=risk_gender_pcts, x='gender', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(ylim=(0,1))
g.fig.suptitle("Risk by Gender (normalized, withdraws removed)")

g = sns.catplot(kind='bar', data=risk_region_pcts, x='region', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(ylim=(0,1))
g.set_xticklabels(rotation=90)
g.fig.suptitle("Risk by Region (normalized, withdraws removed)")

g = sns.catplot(kind='bar', data=risk_ed_pcts, x='highest_education', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(ylim=(0,1))
g.set_xticklabels(rotation=90)
g.fig.suptitle("Risk by Highest Education (normalized, withdraws removed)")

g = sns.catplot(kind='bar', data=risk_age_pcts, x='age_band', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(ylim=(0,1))
g.fig.suptitle("Risk by Age Band (normalized, withdraws removed)")

g = sns.catplot(kind='bar', data=risk_disb_pcts, x='disability', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(ylim=(0,1))
g.fig.suptitle("Risk by Disability Status (normalized, withdraws removed)")

g = sns.catplot(kind='bar', data=risk_term_pcts, x='code_presentation', hue='class', y='pct', hue_order=['not at risk', 'at risk'])
g.set(xlabel="Term")
g.set(ylim=(0,1))
g.fig.suptitle("Risk by Semester Term (normalized, withdraws removed)")
######################################################################################################################################

# Decision Tree classifier
from sklearn.preprocessing import OrdinalEncoder, label_binarize, RobustScaler, robust_scale

from sklearn.tree import DecisionTreeClassifier
from sklearn.ensemble import RandomForestClassifier
from sklearn.linear_model import LogisticRegression
from sklearn.tree import export_graphviz
from sklearn.feature_selection import RFE
from sklearn.model_selection import  StratifiedKFold
from sklearn.metrics import f1_score, accuracy_score, precision_score, recall_score

def perform_run(classifier, X, y, run = 0):
    f1 = np.zeros((5,))
    p = np.zeros((5,))
    r = np.zeros((5,))
    acc = np.zeros((5,))

    skf = StratifiedKFold(n_splits = 5)

    i = 0
    for train_index, test_index in skf.split(X, y):
        X_train, X_test = X[train_index], X[test_index]
        y_train, y_test = y[train_index], y[test_index]
        classifier.fit(X_train, y_train)
        y_pred = classifier.predict(X_test)
        f1[i] = f1_score(y_test, y_pred)
        p[i] = precision_score(y_test, y_pred)
        r[i] = recall_score(y_test, y_pred)
        acc[i] = accuracy_score(y_test, y_pred)
        i += 1
    print("{} {}-------------".format(classifier.__class__.__name__, run))
    print("F1:  {}".format(f1.mean()))
    print("P:   {}".format(p.mean()))
    print("R:   {}".format(r.mean()))
    print("ACC: {}".format(acc.mean()))
    return {'F1': f1.mean(), 'P': p.mean(), 'R': r.mean(), 'ACC': acc.mean()}

# Data preprocessing

# Remove missing values in imd_band
studentinfo_df = studentinfo_df[studentinfo_df['imd_band'] != '?']

# Remove withdrawn students
studentinfo_df_no_wd = remove_withdraws(studentinfo_df)

total = len(studentinfo_df_no_wd)
num_at_risk = len(studentinfo_df_no_wd[studentinfo_df_no_wd['class'] == 'at risk'])

print("Total instances in dataset: {}".format(total))
print("At risk: {}, {}%".format(num_at_risk, float(num_at_risk) / total))
print("Not at risk: {}, {}%".format(total - num_at_risk, float(total - num_at_risk) / total))

classes = ['not at risk', 'at risk']
class_enc = OrdinalEncoder(categories=[classes])

X = studentinfo_df_no_wd.drop(['id_student', 'class', 'final_result'], axis=1)

X = X.drop(['code_module'], axis=1)
X = X.drop(['code_presentation'], axis=1)
X = X.drop(['num_of_prev_attempts'], axis=1)
X = X.drop(['studied_credits'], axis=1)
X = X.drop(['age_band'], axis=1)
#X = X.drop(['region'], axis=1)
#X = X.drop(['gender'], axis=1)
#X = X.drop(['disability'])
#X = X.drop(['highest_education'], axis=1)
#X = X.drop(['imd_band'], axis=1)

categorical_features = ['code_module', 'code_presentation', 'gender', 'region', 'disability']
for feature in categorical_features:
    enc = OrdinalEncoder(categories=[X[feature].unique()])
    X[feature] = enc.fit_transform(X[feature].values.reshape(-1,1))


enc = OrdinalEncoder(categories=[['0-35', '35-55', '55<=']])
X.age_band = enc.fit_transform(X.age_band.values.reshape(-1,1))

enc = OrdinalEncoder(categories=[['No Formal quals', 'HE Qualification', 'Lower Than A Level', 'A Level or Equivalent', 'Post Graduate Qualification']])
X.highest_education = enc.fit_transform(X.highest_education.values.reshape(-1,1))

enc = OrdinalEncoder(categories=[['0-10%', '10-20%', '20-30%', '30-40%', '40-50%', '50-60%', '60-70%', '70-80%', '80-90%', '90-100%']])
X.imd_band = enc.fit_transform(X.imd_band.values.reshape(-1,1))

X = pd.get_dummies(X, prefix_sep = '_')

y = studentinfo_df_no_wd['class']
y = class_enc.fit_transform(y.values.reshape(-1,1))

classifier = RandomForestClassifier(criterion='entropy', n_estimators=100, max_depth=3)
rfe = RFE(classifier)
perform_run(rfe, X.values, y.ravel())

classifier = DecisionTreeClassifier(criterion='entropy')
perform_run(classifier, X.values, y.ravel())

classifier = DecisionTreeClassifier(criterion='entropy', max_depth = 3)
perform_run(classifier, X.values, y.ravel())
export_graphviz(classifier, out_file="/Users/alexlauni/Documents/ODU/CS773/Capstone/dtree.dot",
                class_names = np.unique(class_enc.inverse_transform(y)),
                feature_names = X.columns,
                rounded = True)

#X['studied_credits'] = robust_scale(X['studied_credits'])

classifier = LogisticRegression(solver='liblinear', max_iter=300)
perform_run(classifier, X.values, y.ravel())

## VLE Logistic Regression Classifier

studentvle_vector = studentvle_df_with_risk_no_wd.pivot_table(values='sum_click', index=studentvle_df_with_risk_no_wd.id_student, columns='date', fill_value=0, aggfunc='mean')
studentvle_vector.reset_index(inplace=True)
studentvle_vector = studentvle_vector.join(studentinfo_df_no_wd[['id_student', 'final_result']].set_index('id_student'), on='id_student')
studentvle_vector.dropna(axis=0, inplace=True)
studentvle_vector = set_risk_class(studentvle_vector)

y = studentvle_vector['class']
classes = ['not at risk', 'at risk']
class_enc = OrdinalEncoder(categories=[classes])
y = class_enc.fit_transform(y.values.reshape(-1,1)).reshape((len(y),))

X = studentvle_vector.drop(['id_student', 'final_result', 'class'], axis=1)


classifier = LogisticRegression(solver='liblinear', max_iter=1000)
perform_run(classifier, X.values, y)

from sklearn.svm import SVC
classifier = SVC(gamma='scale')
perform_run(classifier, X.values, y)

X_binary = X.applymap(np.vectorize(lambda x: 1 if x else 0))
perform_run(classifier, X_binary.values, y)

## SOMETHING TO TRY!!!
## how many days of data do we need to get a good classification prediction.
## start with [0 - 7] and increase +1 week of interaction data and compare ability to predict?
runs = {}
for week in range(1, int(np.ceil(len(X.columns)/7 + 1))):
    res = perform_run(classifier, X_binary.values[:,: week * 7], y)
    runs[week] = { stat: res[stat] for stat in ["F1", "ACC"] }

g = sns.lineplot(x=list(runs.keys()), y=[100*v['ACC'] for v in runs.values()])
g.set_xlabel("Week")
g.set_ylabel("Accuracy")
g.set(ylim=(40,100))
g.set(xlim=(1, 43))
g.set_title("Accuracy of VLE interaction predictor by week")
plt.legend(labels=["Withdraw and Failure", "Failure only"])















le = None
for column in df.columns:
    if df[column].dtype == type(object):
        le = LabelEncoder()
        df[column] = le.fit_transform(df[column])

X = df.iloc[:, :-1].values
y = df.iloc[:, -1].values


tprs = []
aucs = []
mean_fpr = np.linspace(0, 1, 100)

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size = 0.5)
tree = DecisionTreeClassifier(criterion="entropy", max_depth = 4)
tree.fit(X_train, y_train)
y_pred = tree.predict(X_test)

import sklearn
sklearn.tree.plot_tree(tree.fit(X, y))

scores = cross_validate(estimator = tree, X = X, y = y, cv = 5, return_estimator = True)
bestimator = scores['estimator'][2]

export_graphviz(bestimator, out_file="dtree.dot",
                class_names = numpy.unique(le.inverse_transform(y)),
                rounded = True)


fpr, tpr, thresholds = roc_curve(y_test, probas)
tprs.append(interp(mean_fpr, fpr, tpr))
tprs[-1][0] = 0.0
roc_auc = auc(fpr, tpr)
aucs.append(roc_auc)
plt.plot(fpr, tpr, lw = 1, alpha = 0.3,
         label = 'ROC fold %d (AUC = %0.2f)' % (i, roc_auc))


plt.plot([0,1], [0,1], linestyle = '--', lw = 2, color = 'r',
         label = 'Chance', alpha = 0.8)

mean_tpr = np.mean(tprs, axis=0)
mean_tpr[-1] = 1.0
mean_auc = auc(mean_fpr, mean_tpr)
std_auc = np.std(aucs)
plt.plot(mean_fpr, mean_tpr, color='b',
         label = r'Mean ROC (AUC = %0.2f $\pm$ %0.2f)' % (mean_auc, std_auc),
         lw = 2, alpha = 0.8)

std_tpr = np.std(tprs, axis=0)
tprs_upper = np.minimum(mean_tpr + std_tpr, 1)
tprs_lower = np.maximum(mean_tpr - std_tpr, 0)
plt.fill_between(mean_fpr, tprs_lower, tprs_upper, color='grey', alpha=.2,
                 label=r'$\pm$ 1 std. dev.')

plt.xlim([-0.05, 1.05])
plt.ylim([-0.05, 1.05])
plt.xlabel('False Positive Rate')
plt.ylabel('True Positive Rate')
plt.title('Receiver operating characteristic example')
plt.legend(loc="lower right")
plt.show()
