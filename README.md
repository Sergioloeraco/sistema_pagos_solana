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

## 🔑 Program ID (`declare_id!`)

En la raíz del contrato (`lib.rs`), el programa está vinculado a una dirección pública específica mediante la macro de Anchor:

```rust
declare_id!("CtjdcPu9eLVSWD5vTKhjXasmviNGccqAojoeDx5CNETX");

## 👨‍💻 Autor
**Sergio Loera** - Desarrollador del Smart Contract.
