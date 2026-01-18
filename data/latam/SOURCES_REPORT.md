# Latin American Spanish Dialect Sources Report
# Generated: January 18, 2026

## Executive Summary

This report documents structured datasets and linguistic resources for Latin American Spanish variants, including South American, Central American, and Caribbean Spanish. Focus is on sources with semantic categories suitable for NLP disambiguation tasks.

---

## 1. HIGH-PRIORITY DATASETS (Structured, Semantic Categories)

### 1.1 MELISA Dataset (lpsc-fiuba/melisa)
**Best for: Regional vocabulary with semantic categories**

- **URL**: https://huggingface.co/datasets/lpsc-fiuba/melisa
- **GitHub**: https://github.com/lpsc-fiuba/MeLiSA
- **Countries**: Argentina (MLA), Colombia (MCO), Peru (MPE), Uruguay (MLU), Chile (MLC), Venezuela (MLV), Mexico (MLM)
- **Size**: 442,108 Spanish samples
- **Format**: Parquet/HuggingFace Datasets
- **Semantic Categories**:
  - Hogar / Casa (Home)
  - Tecnologia y electronica (Technology & Electronics)
  - Salud, ropa y cuidado personal (Health, Clothing & Personal Care)
  - Arte y entretenimiento (Arts & Entertainment)
  - Alimentos y Bebidas (Food & Beverages)

**Download Command**:
```python
from datasets import load_dataset
dataset = load_dataset("lpsc-fiuba/melisa", "es")
# Access by country: dataset.filter(lambda x: x['country'] == 'MLA')
```

### 1.2 DiaWUG: Diatopic Word Usage Graphs
**Best for: Lexical semantic variation across regions**

- **Paper**: https://aclanthology.org/2022.lrec-1.278/
- **Data**: https://zenodo.org/record/5791193
- **DOI**: 10.5281/zenodo.5791193
- **Variants Covered**:
  - 0: Spain (ES)
  - 1: Cuba (CU)
  - 2: Colombia (CO)
  - 3: Argentina (AR)
  - 4: Peru (PE)
  - 6: Venezuela (VE)
- **Features**: Semantic relatedness annotations, polysemy, synonymy
- **Format**: Word Usage Graphs (WUGs)

**Download**: Direct from Zenodo link above.

### 1.3 INGEOTEC Regional Spanish Models
**Best for: Country-specific vocabulary and embeddings**

- **GitHub**: https://github.com/INGEOTEC/regional-spanish-models
- **Embeddings**: https://github.com/INGEOTEC/RegionalEmbeddings
- **Website**: https://ingeotec.github.io/regional-spanish-models/
- **Paper**: https://arxiv.org/abs/2110.06128
- **Countries**: 26 Spanish-speaking countries
- **Data Source**: 586+ million geolocated tweets (2015-2018)
- **Available Models**:
  - FastText word embeddings (4 dimensionalities per country)
  - BILMA language models (8 countries + combined)
  - Country-specific vocabularies
- **Formats**: .bin (binary), .vec (ASCII text)

**Key Features**:
- Regional semantics captured by geography
- Culture-aware embeddings
- Per-country vocabulary files

---

## 2. SPEECH/AUDIO DATASETS (With Text Transcripts)

### 2.1 Spanish Dialects (rjnieto/spanish-dialects)
- **URL**: https://huggingface.co/datasets/rjnieto/spanish-dialects
- **Countries**: Spain, Mexico, Chile, Argentina, Dominican Republic
- **Size**: 10+ hours audio
- **Format**: Parquet (audio + text)
- **Note**: Requires HuggingFace login and terms acceptance

### 2.2 Chilean Spanish (ylacombe/google-chilean-spanish)
- **URL**: https://huggingface.co/datasets/ylacombe/google-chilean-spanish
- **Focus**: Chilean Spanish specifically
- **Use Case**: Low-resource TTS systems

### 2.3 LDC CALLFRIEND Spanish-Caribbean
- **URL**: https://catalog.ldc.upenn.edu/LDC96S57
- **Content**: 60 telephone conversations
- **Regions**: Caribbean Spanish
- **Note**: Paid resource from LDC

### 2.4 Fisher Spanish Speech
- **URL**: https://catalog.ldc.upenn.edu/LDC2010S01
- **Size**: 163 hours, 819 conversations
- **Speakers**: Caribbean + Non-Caribbean Spanish
- **Note**: Paid resource from LDC

---

## 3. CORPORA (Large-Scale Text)

### 3.1 Corpus del Espanol Web/Dialects
- **URL**: https://www.corpusdelespanol.org/
- **Download**: https://www.corpusdata.org/spanish.asp
- **Size**: 2 billion words
- **Countries**: 21 Spanish-speaking countries
- **Features**: Lemmatized, POS-tagged
- **Access**: Registration required, research use

### 3.2 esTenTen (Sketch Engine)
- **URL**: https://www.sketchengine.eu/estenten-spanish-corpus/
- **Size**: 28.6 billion words (2023 version)
- **Subcorpora**: European Spanish, American Spanish
- **Note**: Commercial product, subscription required

### 3.3 SomosNLP Collections
- **URL**: https://huggingface.co/somosnlp
- **Collections**:
  - Corpus: Spanish
  - Evaluation datasets for ES & LATAM
  - Co-official languages
- **Focus**: Dialect diversity from 600M Spanish speakers

---

## 4. SPECIALIZED RESOURCES

### 4.1 Spanish is Not Just One (Dialect Recognition)
- **Paper**: https://www.sciencedirect.com/science/article/pii/S2352340925008108
- **Focus**: LLM dialect recognition evaluation
- **Format**: 30 multiple-choice questions on regional variations
- **Use Case**: Testing LLM dialect awareness

### 4.2 SPALEX: Spanish Lexical Decision Database
- **URL**: https://www.frontiersin.org/articles/10.3389/fpsyg.2018.02156/
- **Features**: Vocabulary prevalence differences Spain vs. Latin America
- **Use Case**: Cultural vocabulary differences

### 4.3 EsPal: Spanish Lexical Database
- **URL**: https://www.bcbl.eu/databases/espal/
- **Features**: Lexical properties, word generation tools

---

## 5. REGIONAL VOCABULARY PATTERNS

### Known Lexical Variations (Examples for Reference)

| Concept | Spain | Mexico | Argentina | Chile | Caribbean |
|---------|-------|--------|-----------|-------|-----------|
| Bus | autobus | camion | colectivo, bondi | micro | guagua |
| You (informal) | tu | tu | vos | tu (verbal voseo) | tu |
| You (plural) | vosotros | ustedes | ustedes | ustedes | ustedes |
| To take | coger | tomar | agarrar | tomar | coger |
| Apartment | piso | departamento | departamento | departamento | apartamento |
| Car | coche | carro, coche | auto | auto | carro |
| Computer | ordenador | computadora | computadora | computador | computadora |
| Popcorn | palomitas | palomitas | pochoclo | cabritas | cotufas |
| Banana | platano | platano | banana | platano | guineo |

---

## 6. DOWNLOAD SCRIPTS

### 6.1 MELISA Dataset
```python
# pip install datasets
from datasets import load_dataset
import os

# Create output directory
os.makedirs("C:/Users/pakom/NL-SRE-Semantico/data/latam/melisa", exist_ok=True)

# Load Spanish subset
ds = load_dataset("lpsc-fiuba/melisa", "es")

# Save by country
for country in ['MLA', 'MCO', 'MPE', 'MLU', 'MLC', 'MLV', 'MLM']:
    subset = ds['train'].filter(lambda x: x['country'] == country)
    subset.to_json(f"C:/Users/pakom/NL-SRE-Semantico/data/latam/melisa/{country}.jsonl")
```

### 6.2 DiaWUG Dataset
```bash
# From Zenodo
curl -L -o diawug.zip "https://zenodo.org/record/5791193/files/DiaWUG.zip?download=1"
unzip diawug.zip -d C:/Users/pakom/NL-SRE-Semantico/data/latam/diawug/
```

### 6.3 INGEOTEC Regional Embeddings
```bash
# Clone repository
git clone https://github.com/INGEOTEC/RegionalEmbeddings.git
# Models available per country (AR, CL, CO, CU, MX, PE, VE, etc.)
# Download specific country embeddings from releases
```

### 6.4 HuggingFace Spanish Dialects
```python
from datasets import load_dataset
# Requires login: huggingface-cli login
ds = load_dataset("rjnieto/spanish-dialects")
```

---

## 7. RECOMMENDED PRIORITY ORDER

For NL-SRE-Semantico disambiguation:

1. **MELISA** - Immediate: 7 Latin American countries with semantic categories
2. **DiaWUG** - Immediate: Lexical semantic variation annotations
3. **INGEOTEC** - Short-term: Country-specific vocabularies and embeddings
4. **SomosNLP** - Medium-term: Growing collection of dialect resources
5. **Corpus del Espanol** - Long-term: Large-scale dialectal corpus

---

## 8. REFERENCES

1. Tellez, E.S., et al. "Regionalized models for Spanish language variations based on Twitter." Language Resources and Evaluation (2023). https://arxiv.org/abs/2110.06128

2. Baldissin, G., Schlechtweg, D., & Schulte im Walde, S. "DiaWUG: A Dataset for Diatopic Lexical Semantic Variation in Spanish." LREC 2022. https://aclanthology.org/2022.lrec-1.278/

3. Lestienne, et al. "MeLiSA: Mercado Libre for Sentiment Analysis." LPSC-FIUBA. https://github.com/lpsc-fiuba/MeLiSA

4. SomosNLP. "The #Somos600M Project: Generating NLP resources that represent the diversity of the languages from LATAM, the Caribbean, and Spain." https://arxiv.org/abs/2407.17479

5. Davies, Mark. "Corpus del Espanol: Web/Dialects." https://www.corpusdelespanol.org/

---

## Contact

Report generated for NL-SRE-Semantico project.
Francisco Molina-Burgos, Avermex Research Division
