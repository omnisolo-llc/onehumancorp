use crate::db::{LocalDb, OfflineMutation};

#[tauri::command]
pub async fn queue_offline_mutation(
    mutation: OfflineMutation,
    db: tauri::State<'_, LocalDb>,
) -> Result<(), String> {
    db.add_mutation(mutation).await
}

#[tauri::command]
pub async fn get_pending_mutations(
    db: tauri::State<'_, LocalDb>,
) -> Result<Vec<OfflineMutation>, String> {
    db.get_pending_mutations().await
}
