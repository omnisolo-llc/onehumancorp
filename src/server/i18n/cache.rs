use std::collections::HashMap;
use std::sync::Mutex;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TranslationData {
    pub translations: HashMap<String, HashMap<String, String>>,
    pub exchange_rates: HashMap<String, f64>,
}

pub struct I18nCache {
    data: Mutex<TranslationData>,
}

impl I18nCache {
    pub fn new() -> Self {
        let mut translations = HashMap::new();

        let mut ar = HashMap::new();
        ar.insert("checkout.pay_now".to_string(), "ادفع الآن".to_string());
        ar.insert("checkout.cancel".to_string(), "إلغاء".to_string());
        ar.insert("checkout.offline_warning".to_string(), "أنت غير متصل بالإنترنت. تمت مزامنة المعاملة باستخدام سعر الأمس.".to_string());
        ar.insert("checkout.title".to_string(), "الدفع".to_string());
        ar.insert("checkout.subtitle".to_string(), "الرجاء إدخال تفاصيل الدفع أدناه.".to_string());
        translations.insert("ar".to_string(), ar);

        let mut es = HashMap::new();
        es.insert("checkout.pay_now".to_string(), "Pagar ahora".to_string());
        es.insert("checkout.cancel".to_string(), "Cancelar".to_string());
        es.insert("checkout.offline_warning".to_string(), "Estás desconectado. Convertido con la tasa de ayer. Se finalizará en la sincronización.".to_string());
        es.insert("checkout.title".to_string(), "Pagar".to_string());
        es.insert("checkout.subtitle".to_string(), "Ingrese sus detalles de pago a continuación.".to_string());
        translations.insert("es".to_string(), es);

        let mut en = HashMap::new();
        en.insert("checkout.pay_now".to_string(), "Pay Now".to_string());
        en.insert("checkout.cancel".to_string(), "Cancel".to_string());
        en.insert("checkout.offline_warning".to_string(), "Converted using yesterday's rate. Will finalize on sync.".to_string());
        en.insert("checkout.title".to_string(), "Checkout".to_string());
        en.insert("checkout.subtitle".to_string(), "Please enter your payment details below.".to_string());
        translations.insert("en".to_string(), en);

        let mut exchange_rates = HashMap::new();
        exchange_rates.insert("USD".to_string(), 1.0);
        exchange_rates.insert("EUR".to_string(), 0.92);
        exchange_rates.insert("AED".to_string(), 3.67);
        exchange_rates.insert("BRL".to_string(), 5.15);
        exchange_rates.insert("GBP".to_string(), 0.79);

        Self {
            data: Mutex::new(TranslationData {
                translations,
                exchange_rates,
            }),
        }
    }

    pub fn get_data(&self) -> TranslationData {
        self.data.lock().unwrap().clone()
    }
}
