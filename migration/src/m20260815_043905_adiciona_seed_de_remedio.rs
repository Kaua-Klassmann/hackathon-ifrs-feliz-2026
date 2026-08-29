use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260815_043905_adiciona_seed_de_remedio"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Alias::new("remedies"))
                    .columns([Alias::new("name")])
                    .values_panic(["Hidroclorotiazida".into()])
                    .values_panic(["Losartana".into()])
                    .values_panic(["Sinvastatina".into()])
                    .values_panic(["Metformina".into()])
                    .values_panic(["Enalapril".into()])
                    .values_panic(["Captopril".into()])
                    .values_panic(["Atenolol".into()])
                    .values_panic(["Anlodipino".into()])
                    .values_panic(["Ácido acetilsalicílico".into()])
                    .values_panic(["Glibenclamida".into()])
                    .values_panic(["Omeprazol".into()])
                    .values_panic(["Furosemida".into()])
                    .values_panic(["Levotiroxina".into()])
                    .values_panic(["Atorvastatina".into()])
                    .values_panic(["Clonazepam".into()])
                    .values_panic(["Paracetamol".into()])
                    .values_panic(["Dipirona".into()])
                    .values_panic(["Insulina humana".into()])
                    .values_panic(["Atenolol".into()])
                    .values_panic(["Propranolol".into()])
                    .values_panic(["Nifedipino".into()])
                    .values_panic(["Verapamil".into()])
                    .values_panic(["Diltiazem".into()])
                    .values_panic(["Carvedilol".into()])
                    .values_panic(["Metoprolol".into()])
                    .values_panic(["Espironolactona".into()])
                    .values_panic(["Digoxina".into()])
                    .values_panic(["Enalapril + Hidroclorotiazida".into()])
                    .values_panic(["Losartana + Hidroclorotiazida".into()])
                    .values_panic(["Gliclazida".into()])
                    .values_panic(["Glimepirida".into()])
                    .values_panic(["Amitriptilina".into()])
                    .values_panic(["Fluoxetina".into()])
                    .values_panic(["Sertralina".into()])
                    .values_panic(["Diazepam".into()])
                    .values_panic(["Alendronato".into()])
                    .values_panic(["Carbonato de cálcio".into()])
                    .values_panic(["Vitamina D".into()])
                    .values_panic(["Pantoprazol".into()])
                    .values_panic(["Diclofenaco".into()])
                    .values_panic(["Ibuprofeno".into()])
                    .values_panic(["Tramadol".into()])
                    .values_panic(["Clopidogrel".into()])
                    .values_panic(["Varfarina".into()])
                    .values_panic(["Rivaroxabana".into()])
                    .values_panic(["Tansulosina".into()])
                    .values_panic(["Finasterida".into()])
                    .values_panic(["Donepezila".into()])
                    .values_panic(["Memantina".into()])
                    .values_panic(["Amoxicilina".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::delete()
                    .from_table(Alias::new("remedies"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
