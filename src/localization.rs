use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    En,
    Es,
    Tr,
    Ru,
    De,
    Fr,
    EnUk,
    EnNz,
}

impl Default for Language {
    fn default() -> Self {
        Language::En
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Language::En => "en",
            Language::Es => "es",
            Language::Tr => "tr",
            Language::Ru => "ru",
            Language::De => "de",
            Language::Fr => "fr",
            Language::EnUk => "en_UK",
            Language::EnNz => "en_NZ",
        };
        write!(f, "{}", code)
    }
}

impl Language {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "en" => Some(Language::En),
            "es" => Some(Language::Es),
            "tr" => Some(Language::Tr),
            "ru" => Some(Language::Ru),
            "de" => Some(Language::De),
            "fr" => Some(Language::Fr),
            "en_UK" => Some(Language::EnUk),
            "en_NZ" => Some(Language::EnNz),
            _ => None,
        }
    }
}

pub struct Translator;

impl Translator {
    pub fn start_message(lang: Language) -> &'static str {
        match lang {
            Language::En => "Welcome! Please select your language or start using commands.",
            Language::Es => "¡Bienvenido! Seleccione su idioma o comience a usar los comandos.",
            Language::Tr => "Hoş geldiniz! Lütfen dilinizi seçin veya komutları kullanmaya başlayın.",
            Language::Ru => "Добро пожаловать! Выберите язык или начните использовать команды.",
            Language::De => "Willkommen! Bitte wählen Sie Ihre Sprache.",
            Language::Fr => "Bienvenue ! Veuillez choisir votre langue.",
            Language::EnUk => "Welcome! Please select your language, mate.",
            Language::EnNz => "Sweet as, bro! Choose your lingo or just rip into it.",
        }
    }

    pub fn language_updated(lang: Language) -> &'static str {
        match lang {
            Language::En => "Language updated to English.",
            Language::Es => "Idioma actualizado a Español.",
            Language::Tr => "Dil Türkçe olarak güncellendi.",
            Language::Ru => "Язык изменен на русский.",
            Language::De => "Sprache auf Deutsch aktualisiert.",
            Language::Fr => "Langue mise à jour en français.",
            Language::EnUk => "Language updated to British English. Brilliant!",
            Language::EnNz => "Chur! We're rolling with Kiwi English now. Choice, choice!",
        }
    }

    pub fn media_choice_prompt(lang: Language) -> &'static str {
        match lang {
            Language::En => "Select format for download:",
            Language::Es => "Seleccione el formato de descarga:",
            Language::Tr => "İndirme formatını seçin:",
            Language::Ru => "Выберите формат для загрузки:",
            Language::De => "Wählen Sie das Download-Format:",
            Language::Fr => "Sélectionnez le format de téléchargement :",
            Language::EnUk => "Select format for download, please:",
            Language::EnNz => "Yeah nah, let's download this instead. Pick a format, mate:",
        }
    }

    pub fn sarcastic_url_fallback(lang: Language) -> &'static str {
        match lang {
            Language::En => "We can certainly do that, but please use /video for links. Isn't that easier? 🙃",
            Language::Es => "Podemos hacerlo, pero por favor usa /video para enlaces. ¿No es más fácil así? 🙃",
            Language::Tr => "Bunu kesinlikle yapabiliriz ama linkler için /video kullanmak daha iyi olmaz mı? 🙃",
            Language::Ru => "Мы конечно можем, но просим выбирать через /, так удобнее, разве нет ¿🙃?",
            Language::De => "Wir können das natürlich machen, aber bitte benutze /video für Links. Ist das nicht einfacher? 🙃",
            Language::Fr => "On peut le faire bien sûr, mais utilise /video pour les liens. C'est plus pratique, non ? 🙃",
            Language::EnUk => "We can manage that, but do use /video next time. Bit more sensible, isn't it? 🙃",
            Language::EnNz => "Yeah nah we can sort that out, but chuck it in /video next time eh? Saves us all a headache. 🙃",
        }
    }

    pub fn invalid_id(lang: Language) -> &'static str {
        match lang {
            Language::En => "Invalid ID format.",
            Language::Es => "Formato de ID inválido.",
            Language::Tr => "Geçersiz ID formatı.",
            Language::Ru => "Неверный формат ID.",
            Language::De => "Ungültiges ID-Format.",
            Language::Fr => "Format d'ID invalide.",
            Language::EnUk => "Invalid ID format, sorry.",
            Language::EnNz => "Nah mate, that ID is bung.",
        }
    }
}
