from rdkit import Chem

def sanitize(rdmol, options):
    Chem.SanitizeMol(rdmol, options)
