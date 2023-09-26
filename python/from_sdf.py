from rdkit import Chem


def from_sdf(filename):
    sdf_supplier = Chem.ForwardSDMolSupplier(
        filename, removeHs=False, sanitize=False, strictParsing=True
    )
    mols = []
    for rdmol in sdf_supplier:
        if rdmol is None:
            continue

        # Sanitize the molecules (fails on nitro groups)
        try:
            Chem.SanitizeMol(
                rdmol,
                Chem.SANITIZE_ALL
                ^ Chem.SANITIZE_SETAROMATICITY
                ^ Chem.SANITIZE_ADJUSTHS,
            )
            Chem.AssignStereochemistryFrom3D(rdmol)
        except ValueError:
            continue
        Chem.SetAromaticity(rdmol, Chem.AromaticityModel.AROMATICITY_MDL)
        mols.append(rdmol)
    if len(mols) > 1:
        print("warning: multiple molecules found, only 1 returned")
    return mols[0]
