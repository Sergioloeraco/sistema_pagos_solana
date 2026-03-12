# 💸 Sistema de Pagos Descentralizado en Solana (Smart Contract)

Este repositorio contiene el backend (Smart Contract) para un sistema de enlaces de pago Web3 construido en la blockchain de Solana. El objetivo de este contrato es eliminar a los intermediarios en transacciones digitales, permitiendo a los comercios generar links de cobro y recibir el saldo directamente en sus billeteras (Peer-to-Peer) con comisiones mínimas de red y liquidación en segundos.

## 🛠️ Tecnologías Utilizadas

* **Lenguaje:** Rust
* **Framework:** Anchor (v0.29.0+)
* **Red:** Solana Devnet
* **Entorno de Desarrollo:** Solana Playground

## 🏗️ Arquitectura y Estado (PDAs)

## 🏗️ Arquitectura C.R.U.D. y Estado (PDAs)

Este proyecto implementa una arquitectura **C.R.U.D. (Create, Read, Update, Delete)** 100% nativa en Web3. A diferencia de las bases de datos tradicionales, este contrato utiliza **Program Derived Addresses (PDAs)** para almacenar el estado de cada cobro de manera inmutable en la blockchain.

El esquema de datos (`PaymentState`) reserva espacio específico para los siguientes campos:

* `authority` (Pubkey): La llave pública del creador del cobro (quien recibe los fondos).
* `id` (String): Identificador único de la factura (ej. "factura-001").
* `amount` (u64): El monto a cobrar, expresado en Lamports.
* `description` (String): El concepto del pago.
* `is_paid` (bool): Bandera que indica si el cobro ya fue liquidado.

## 🔑 Program ID (`declare_id!`)

En la raíz del contrato (`lib.rs`), el programa está vinculado a una dirección pública específica mediante la macro de Anchor:

```rust
declare_id!("3s1VNMLu4ahyqT1FwxxCsPNH9hfBAFyWNGkhwPApswMD");
```

## ⚙️ Instrucciones del Programa

El Smart Contract expone dos métodos principales (RPC endpoints):

### 1. `create_payment` (Create)
Permite a un usuario (comercio) inicializar un nuevo cobro.
* Deriva un PDA único usando la semilla `b"payment"`, la `authority` y el `id` de la factura.
* Asigna los valores iniciales y establece `is_paid = false`.
* El creador paga por el alquiler del espacio de almacenamiento en la red (Rent).

### 2. Lectura de Estado (Read)
* El frontend cliente interactúa directamente con la cuenta `PaymentState` consultando la blockchain en tiempo real para extraer los datos de la factura antes de procesar el pago.

### 3. `pay` (Update)
Permite a un cliente liquidar una factura existente.
* Localiza el PDA exacto mediante las semillas.
* Verifica que el estado actual sea `is_paid == false` para evitar cobros dobles (Manejo de errores: `AlreadyPaid`).
* Ejecuta una transferencia cruzada (Cross-Program Invocation - CPI) usando el `SystemProgram` para mover los fondos del cliente a la cuenta del comercio.
* Actualiza el estado del PDA a `is_paid = true.`

### 4. `deletePayment` (Delete)
* Permite al creador del link eliminar el registro si hubo un error o si ya no es necesario.
* Valida criptográficamente mediante `has_one = authority` que solo el dueño legítimo pueda borrar la factura.
* Utiliza el modificador `close` de Anchor para destruir el PDA inmutablemente.
* Reembolsa automáticamente los Lamports del Rent (alquiler de espacio) de vuelta a la billetera del comercio.

## 🚀 Cómo probar en Solana Playground

Dado que este proyecto fue desarrollado nativamente para el ecosistema de Solana, la forma más rápida de probarlo es utilizando [Solana Playground](https://beta.solpg.io/).

1. Abre Solana Playground y crea un nuevo proyecto seleccionando el framework **Anchor (Rust)**.
2. Copia el contenido del archivo `lib.rs` de este repositorio y pégalo en tu entorno.
3. Conecta tu billetera de desarrollo (ej. Phantom en Devnet) en la esquina inferior izquierda.
4. En la pestaña **Build & Deploy**:
   * Haz clic en **Build** para compilar el contrato.
   * Haz clic en **Deploy** para subirlo a la Devnet.
5. Pruebas UI (Test Tab): Una vez desplegado, puedes ir a la pestaña del ícono de matraz (Test). Ahí verás mapeadas automáticamente tus Instructions (createPayment, pay, deletePayment) y Accounts (PaymentState) para interactuar con ellas directamente desde la interfaz gráfica.

> 🔍 **Evidencia de Despliegue:** Puedes verificar este Smart Contract ejecutándose en vivo, así como su historial de transacciones, directamente en el **[Solana Explorer (Devnet)](https://explorer.solana.com/address/3s1VNMLu4ahyqT1FwxxCsPNH9hfBAFyWNGkhwPApswMD?cluster=devnet)**.

## 👨‍💻 Autor
**Sergio Loera** - Desarrollador del Smart Contract.
