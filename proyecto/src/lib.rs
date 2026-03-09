use anchor_lang::prelude::*;
use anchor_lang::system_program;

// Este ID se generará automáticamente en Solana Playground
declare_id!("CtjdcPu9eLVSWD5vTKhjXasmviNGccqAojoeDx5CNETX");

#[program]
pub mod solana_payment_links {
    use super::*;

    // Función para crear un "Link de Pago" en la blockchain
    pub fn create_payment(
        ctx: Context<CreatePayment>,
        id: String,
        amount: u64,
        description: String,
    ) -> Result<()> {
        let payment = &mut ctx.accounts.payment;

        payment.authority = ctx.accounts.authority.key(); // Creador del link
        payment.id = id; // Identificador único (ej. "factura-001")
        payment.amount = amount; // Monto en Lamports (1 SOL = 1,000,000,000 Lamports)
        payment.description = description; // Descripción del pago
        payment.is_paid = false; // Estado inicial

        msg!(
            "Pago creado: {} por {} lamports",
            payment.id,
            payment.amount
        );
        Ok(())
    }

    // Función que ejecuta el cliente al entrar al Link para pagar
    pub fn pay(ctx: Context<Pay>, _id: String) -> Result<()> {
        let payment = &mut ctx.accounts.payment;

        // Verificamos que no esté pagado ya
        require!(!payment.is_paid, PaymentError::AlreadyPaid);

        // Transferir los fondos del cliente (payer) al creador del link (authority)
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.authority.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, payment.amount)?;

        // Actualizar el estado en la "base de datos" de la blockchain
        payment.is_paid = true;

        msg!("Pago completado con éxito");
        Ok(())
    }
}

// ----------------------------------------------------
// DEFINICIÓN DE LAS CUENTAS (Las "Tablas" de la Base de Datos)
// ----------------------------------------------------

#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreatePayment<'info> {
    // Inicializamos el PDA que guardará los datos
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 36 + 8 + 104 + 1, // Espacio reservado en bytes (discriminador + pubkey + id + amount + desc + bool)
        seeds =[b"payment", authority.key().as_ref(), id.as_bytes()], // Semillas para generar la "Llave Primaria"
        bump
    )]
    pub payment: Account<'info, PaymentState>,

    #[account(mut)]
    pub authority: Signer<'info>, // Quien crea y paga el espacio del cobro

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct Pay<'info> {
    // Buscamos el PDA exacto usando las semillas
    #[account(
        mut,
        seeds =[b"payment", authority.key().as_ref(), id.as_bytes()],
        bump,
        has_one = authority, // Seguridad: Garantiza que la autoridad sea la dueña del cobro
    )]
    pub payment: Account<'info, PaymentState>,

    /// CHECK: Es la cuenta que recibe el dinero (no necesitamos ejecutar código en ella)
    #[account(mut)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>, // El cliente que está pagando

    pub system_program: Program<'info, System>,
}

// ----------------------------------------------------
// ESTRUCTURA DE ESTADO (El "Esquema" de la Base de Datos)
// ----------------------------------------------------

#[account]
pub struct PaymentState {
    pub authority: Pubkey,   // 32 bytes (A quién se le paga)
    pub id: String,          // 4 + 32 bytes (ID único)
    pub amount: u64,         // 8 bytes (Monto)
    pub description: String, // 4 + 100 bytes (Concepto)
    pub is_paid: bool,       // 1 byte (Status)
}

// ----------------------------------------------------
// MANEJO DE ERRORES
// ----------------------------------------------------

#[error_code]
pub enum PaymentError {
    #[msg("Este link de pago ya ha sido liquidado.")]
    AlreadyPaid,
}
