import pandas as pd
import numpy as np
import seaborn as sns
import matplotlib.pyplot as plt
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import LabelEncoder
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import accuracy_score, confusion_matrix, classification_report
from imblearn.over_sampling import SMOTE

# Set the aesthetic style of the plots
sns.set_style("whitegrid")

# Load the dataset
dataset_path = '/home/tactical/Downloads/ipynbproject/archive/dataset excel Test.csv'
df = pd.read_csv(dataset_path)

# Drop empty columns and those starting with "Naive_Bayes"
df_cleaned = df.dropna(axis=1, how='all')
df_cleaned = df_cleaned[df_cleaned.columns.drop(df_cleaned.filter(regex='Naive_Bayes'))]

# Overview of the target variable distribution
target_distribution = df_cleaned['Card_Category'].value_counts(normalize=True)
print(target_distribution)

# Distribution of "Customer_Age"
plt.figure(figsize=(10, 6))
sns.histplot(df_cleaned['Customer_Age'], kde=True, bins=30)
plt.title('Distribution of Customer Age')
plt.show()

# Distribution of "Card_Category"
plt.figure(figsize=(10, 6))
sns.countplot(x='Card_Category', data=df_cleaned, order = df_cleaned['Card_Category'].value_counts().index)
plt.title('Distribution of Card Categories')
plt.show()

# Encoding categorical variables
label_encoders = {}
for column in df_cleaned.select_dtypes(include=['object']).columns:
    if column != 'Card_Category':  # Exclude the target variable from encoding
        le = LabelEncoder()
        df_cleaned[column] = le.fit_transform(df_cleaned[column])
        label_encoders[column] = le

# Prepare data for logistic regression
X = df_cleaned.drop('Card_Category', axis=1)
y = df_cleaned['Card_Category']
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Apply SMOTE to balance the classes
smote = SMOTE(random_state=42)
X_train_smote, y_train_smote = smote.fit_resample(X_train, y_train)

# Train logistic regression model
log_reg = LogisticRegression(max_iter=1000)
log_reg.fit(X_train_smote, y_train_smote)

# Predictions
y_pred = log_reg.predict(X_test)

# Evaluation
accuracy = accuracy_score(y_test, y_pred)
conf_matrix = confusion_matrix(y_test, y_pred)

# Assuming df_cleaned contains the original 'Card_Category' column with categorical labels
card_categories = df_cleaned['Card_Category'].unique()

# Confusion Matrix Visualization with direct labels
plt.figure(figsize=(10, 7))
sns.heatmap(conf_matrix, annot=True, fmt='d', cmap='Blues', xticklabels=sorted(card_categories), yticklabels=sorted(card_categories))
plt.xlabel('Predicted Label')
plt.ylabel('True Label')
plt.title('Confusion Matrix')
plt.show()


