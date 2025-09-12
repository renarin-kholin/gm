use std::sync::{atomic::AtomicBool, mpsc, Arc};

use crossterm::event::{KeyCode, KeyEventKind};
use gm_utils::disk::{Config, DiskInterface};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{
    app::{
        pages::{assets::AssetsPage, trade::TradePage, Page},
        widgets::cursor::Cursor,
        SharedState,
    },
    traits::{Component, HandleResult},
    Event,
};

use super::select::Select;

#[derive(Default)]
pub struct Sidebar {
    // pub focus: bool,
    pub cursor: Cursor,
}

impl Component for Sidebar {
    fn handle_event(
        &mut self,
        event: &crate::Event,
        _area: Rect,
        transmitter: &mpsc::Sender<Event>,
        _shutdown_signal: &Arc<AtomicBool>,
        shared_state: &SharedState,
    ) -> crate::Result<HandleResult> {
        self.cursor
            .handle(event, if shared_state.testnet_mode { 2 } else { 3 });

        let mut result = HandleResult::default();

        if let Event::Input(key_event) = event {
            if key_event.kind == KeyEventKind::Press {
                #[allow(clippy::single_match)]
                match key_event.code {
                    KeyCode::Enter => match self.cursor.current {
                        0 => result.page_inserts.push(Page::Trade(TradePage::default())),
                        1 => {
                            let mut config = Config::load()?;
                            config.testnet_mode = !shared_state.testnet_mode;
                            config.save()?;
                            transmitter.send(Event::ConfigUpdate)?;

                            result.reload = true;
                            result.refresh_assets = true;
                        }
                        2 => {
                            result
                                .page_inserts
                                .push(Page::Assets(AssetsPage::default()));
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        Ok(result)
    }

    fn render_component(&self, area: Rect, buf: &mut Buffer, shared_state: &SharedState) -> Rect
    where
        Self: Sized,
    {
        let eth_price = if let Some(eth_price) = &shared_state.eth_price {
            eth_price.clone()
        } else {
            match shared_state.online {
                Some(true) | None => "Loading...".to_string(),
                Some(false) => "Unable to fetch".to_string(),
            }
        };

        let mut list = vec![
            format!("EthPrice: {eth_price}"),
            format!("Testnet Mode: {}", shared_state.testnet_mode),
        ];

        if !shared_state.testnet_mode
            && shared_state.current_account.is_some()
            && shared_state.alchemy_api_key_available
        {
            let portfolio = if let Some(assets) = &shared_state.assets_read().ok().flatten() {
                let portfolio = assets
                    .iter()
                    .fold(0.0, |acc, asset| acc + asset.usd_value().unwrap_or(0.0));

                format!("Portfolio: ${portfolio}")
            } else {
                "Portfolio: Loading...".to_string()
            };

            list.push(portfolio);
        }

        Select {
            // @dev make sure to update event handlers
            list: &list,
            cursor: &self.cursor,
            focus: false,
            focus_style: shared_state.theme.select(),
        }
        .render(area, buf);

        area
    }
}
