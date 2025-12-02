# ⚡ MultiversX Crowdfunding Smart Contract

![Blockchain](https://img.shields.io/badge/Blockchain-MultiversX-blue) ![Language](https://img.shields.io/badge/Language-Rust-orange) ![License](https://img.shields.io/badge/License-MIT-green)

Aquest és un Smart Contract de micromecenatge (Crowdfunding) descentralitzat construït sobre la blockchain de **MultiversX**. Permet als usuaris contribuir amb EGLD a un projecte fins a assolir un objectiu o una data límit, amb regles de seguretat específiques per a les aportacions.

## 📋 Descripció del Projecte

El contracte permet recaptar fons de manera segura i transparent. Els fons queden bloquejats al contracte fins que es compleix la data límit.

La lògica es basa en tres estats possibles:
1.  **FundingPeriod:** El projecte està actiu i accepta donacions.
2.  **Successful:** S'ha assolit l'objectiu (`target`). El propietari pot retirar els fons.
3.  **Failed:** S'ha superat la data límit sense arribar a l'objectiu. Els donants poden recuperar els seus diners.

## 🛡️ Regles de Seguretat i Límits

Aquest contracte implementa controls avançats per garantir una distribució justa i segura:

* **Objectiu (Target):** La quantitat mínima necessària perquè el projecte tingui èxit.
* **Data Límit (Deadline):** Temps màxim per assolir l'objectiu.
* **Hard Cap (Max Cap):** Un sostre màxim de recaptació. Si s'arriba a aquesta quantitat, el contracte no accepta més diners, encara que no hagi passat la data límit.
* **Mínim per Transacció:** Evita "spam" de micro-transaccions exigint una aportació mínima.
* **Màxim per Usuari:** Evita que una sola "balena" acapari tot el projecte limitant la quantitat total que una mateixa adreça pot aportar.

## 🚀 Funcions Principals (Endpoints)

### `init` (Constructor)
S'executa al desplegar el contracte. Configura:
* `target`: Objectiu a assolir.
* `deadline`: Data final (timestamp).
* `min_contribution`: Mínim per transacció.
* `max_per_user`: Màxim acumulat per usuari.
* `max_cap`: Màxim total del projecte.

### `fund` (Payable)
Permet als usuaris enviar EGLD.
* Verifica que estem dins del termini.
* Verifica que l'import > `min_contribution`.
* Verifica que el total del contracte no superi el `max_cap`.
* Verifica que l'usuari no superi el seu `max_per_user`.

### `claim`
Es pot cridar un cop finalitzat el termini:
* **Si ha tingut èxit:** Només l'**owner** pot reclamar tots els EGLD recaptats.
* **Si ha fallat:** Qualsevol **usuari** pot reclamar el reemborsament íntegre de la seva aportació (`deposit`).

### `status` (View)
Retorna l'estat actual del projecte: `FundingPeriod`, `Successful` o `Failed`.

---

## 🛠️ Desenvolupament i Desplegament

### Prerequisits
* [Rust](https://www.rust-lang.org/)
* [MultiversX SDK (sc-meta)](https://docs.multiversx.com/developers/meta/sc-meta)

### Compilació
Per generar el fitxer WASM necessari per al desplegament:

```bash
sc-meta all build
```

## 📜 Llicència

Aquest projecte es distribueix sota la **Llicència MIT**.

Això significa que ets lliure d'utilitzar, modificar i distribuir el codi, tant per a ús privat com comercial, sempre que es mantingui l'atribució original.

Consulta el fitxer `LICENSE` per a més detalls.

## 👤 Autor

* **José Diego Cervellera Forcadell** - *Desenvolupament Smart Contract*
* **GitHub:** [https://[https://github.com/JoseDiego76t](https://github.com/JoseDiego76t)

Si t'ha agradat el projecte o tens dubtes, no dubtis a contactar-me!

