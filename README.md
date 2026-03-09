# 💸 Sistema de Pagos Descentralizado en Solana (Smart Contract)

Este repositorio contiene el backend (Smart Contract) para un sistema de enlaces de pago Web3 construido en la blockchain de Solana. El objetivo de este contrato es eliminar a los intermediarios en transacciones digitales, permitiendo a los comercios generar links de cobro y recibir el saldo directamente en sus billeteras (Peer-to-Peer) con comisiones mínimas de red y liquidación en segundos.

## 🛠️ Tecnologías Utilizadas

* **Lenguaje:** Rust
* **Framework:** Anchor (v0.29.0+)
* **Red:** Solana Devnet
* **Entorno de Desarrollo:** Solana Playground

## 🏗️ Arquitectura y Estado (PDAs)

A diferencia de las bases de datos tradicionales, este contrato utiliza **Program Derived Addresses (PDAs)** para almacenar el estado de cada cobro de manera inmutable en la blockchain.

El esquema de datos (`PaymentState`) reserva espacio específico para los siguientes campos:

* `authority` (Pubkey): La llave pública del creador del cobro (quien recibe los fondos).
* `id` (String): Identificador único de la factura (ej. "factura-001").
* `amount` (u64): El monto a cobrar, expresado en Lamports.
* `description` (String): El concepto del pago.
* `is_paid` (bool): Bandera que indica si el cobro ya fue liquidado.

## ⚙️ Instrucciones del Programa

El Smart Contract expone dos métodos principales (RPC endpoints):

### 1. `create_payment`
Permite a un usuario (comercio) inicializar un nuevo cobro.
* Deriva un PDA único usando la semilla `b"payment"`, la `authority` y el `id` de la factura.
* Asigna los valores iniciales y establece `is_paid = false`.
* El creador paga por el alquiler del espacio de almacenamiento en la red (Rent).

### 2. `pay`
Permite a un cliente liquidar una factura existente.
* Localiza el PDA exacto mediante las semillas.
* Verifica que el estado actual sea `is_paid == false` para evitar cobros dobles (Manejo de errores: `AlreadyPaid`).
* Ejecuta una transferencia cruzada (Cross-Program Invocation - CPI) usando el `SystemProgram` para mover los fondos del cliente a la cuenta del comercio.
* Actualiza el estado del PDA a `is_paid = true`.

## 🚀 Cómo probar en Solana Playground

Dado que este proyecto fue desarrollado nativamente para el ecosistema de Solana, la forma más rápida de probarlo es utilizando [Solana Playground](https://beta.solpg.io/).

1. Abre Solana Playground y crea un nuevo proyecto seleccionando el framework **Anchor (Rust)**.
2. Copia el contenido del archivo `lib.rs` de este repositorio y pégalo en tu entorno.
3. Conecta tu billetera de desarrollo (ej. Phantom en Devnet) en la esquina inferior izquierda.
4. En la pestaña **Build & Deploy**:
   * Haz clic en **Build** para compilar el contrato.
   * Haz clic en **Deploy** para subirlo a la Devnet.
5. (Opcional) Exporta el archivo `idl.json` generado para integrarlo con cualquier frontend cliente (ej. Next.js).

## 👨‍💻 Autor
**Sergio Loera** - Desarrollador del Smart Contract.
