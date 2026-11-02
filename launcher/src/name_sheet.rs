//! The first run's name: with offline play on (`backend::Settings::offline`), no Microsoft
//! account and no offline name, the launcher asks what to call the player and starts the game
//! from the answer. Shown until the player answers, or says Not now (`Settings::name_asked`).
use super::servers_page::Field;
use super::*;
use horizon_launcher::account::Player;

impl Launcher {
    /// Whether the sheet covers the window: nothing to play as yet, and the question not answered.
    pub(crate) fn asking_name(&self) -> bool {
        name_wanted(&self.settings, &self.accounts)
            && !self.asking_owner()
            && !self.asking_consent()
            && self.opening.is_none()
    }

    /// Keeps the typed name and plays: the game starts as that offline name, with no account.
    pub(super) fn choose_name(&mut self) {
        let name = self.servers.value(Field::OfflineName).trim().to_string();
        if name.is_empty() {
            return;
        }
        self.play_as(Player::Offline(name));
        // A refused name says why under the field, and the sheet stays.
        if matches!(self.sign_in_state, Some(SignInState::Done(Err(_)))) {
            return;
        }
        self.servers.text(Field::OfflineName).set(String::new());
        self.settings.name_asked = true;
        self.settings.save();
        self.play_after_name();
    }

    /// Play, as Home's button does it: a version not prepared yet is prepared first, and plays
    /// when that ends well (`play_when_prepared`).
    fn play_after_name(&mut self) {
        let unprepared = self
            .selected()
            .filter(|tag| !backend::is_prepared(tag))
            .cloned();
        match unprepared {
            Some(tag) => {
                self.play_when_prepared = Some(tag);
                self.prepare(backend::MinecraftFrom::Install);
            }
            None => self.play(),
        }
    }

    /// Asks no more: the game makes up a name of its own, as it does without `--name`.
    fn skip_name(&mut self) {
        self.servers.text(Field::OfflineName).set(String::new());
        self.settings.name_asked = true;
        self.settings.save();
    }

    /// The sheet, over the page: the name, Play, and Not now.
    pub(crate) fn name_screen(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let t = self.theme;
        let n = t.night;
        let line = |s: &str, color: u32| {
            shell::text(theme::night::UI, shell::c(color))
                .relative()
                .child(tr(s).to_string())
        };
        let refusal = match &self.sign_in_state {
            Some(SignInState::Done(Err(e))) => Some(e.clone()),
            _ => None,
        };
        let mut body = div()
            .relative()
            .flex()
            .flex_col()
            .gap(px(10.))
            .child(
                shell::text(theme::night::CAPTION, shell::c(n.muted))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(tr("Offline play").to_string()),
            )
            .child(
                shell::text(theme::night::TITLE, shell::c(n.ink))
                    .mb(px(4.))
                    .child(tr("What should we call you?").to_string()),
            )
            .child(line(
                "Horizon plays without a Microsoft account: type the name you want and press Play.",
                n.ink_soft,
            ))
            .child(line(
                "Online servers still need a Microsoft account that owns Minecraft: Java Edition.",
                n.muted,
            ))
            .child(
                div()
                    .relative()
                    .mt(px(10.))
                    .child(self.field(Field::OfflineName, tr("A name, 3 to 16 letters"), 300., cx)),
            );
        if let Some(e) = refusal {
            body = body.child(
                shell::text(theme::night::UI, shell::c(n.danger))
                    .relative()
                    .child(e),
            );
        }
        let answers = div()
            .relative()
            .flex()
            .items_center()
            .gap(px(10.))
            .mt(px(14.))
            .child(
                shell::action("name-play", shell::Action::Main, &t)
                    .child(shell::label(tr("Play").to_string()))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.choose_name();
                        cx.notify();
                    })),
            )
            .child(
                shell::action("name-later", shell::Action::Quiet, &t)
                    .child(shell::label(tr("Not now").to_string()))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.skip_name();
                        cx.notify();
                    })),
            );
        // Opaque: it sits over the page's content, not over the bare sky.
        let card = plate::cut(
            theme::night::PIXEL_CORNER,
            2,
            Some(plate::alpha(n.card, 0.98)),
            Some((plate::alpha(n.stroke, 2. * n.stroke_alpha), 1.)),
        )
        .flex()
        .flex_col()
        .gap(px(8.))
        // A set width: text measured at it wraps where it is drawn.
        .w(px(520.))
        .max_w_full()
        .p(px(32.))
        .child(body)
        .child(answers);
        // The sky stays behind, dimmed: the sheet is over the launcher, not instead of it.
        div()
            .id("name-sheet")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .px(px(32.))
            .py(px(24.))
            .bg(plate::alpha(n.page, 0.8))
            .on_scroll(|_, _, cx| cx.stop_propagation())
            .occlude()
            .child(card)
    }
}

/// Whether the first run still has to ask for a name, whatever the window is doing.
pub(super) fn name_wanted(settings: &Settings, accounts: &Accounts) -> bool {
    settings.offline()
        && !settings.name_asked
        && accounts.accounts.is_empty()
        && accounts.offline.is_empty()
}

/// The computer's user name when an offline name may be it (3 to 16 letters, digits or `_`).
pub(super) fn suggested_name() -> Option<String> {    let raw = ["USERNAME", "USER", "LOGNAME"]
        .iter()
        .find_map(|key| std::env::var(key).ok())?;
    let cleaned: String = raw
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .take(16)
        .collect();
    (cleaned.len() >= 3).then_some(cleaned)
}
