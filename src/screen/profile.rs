mod game;
use game::Game;

mod ranked_overview;
use ranked_overview::RankedOverview;

mod summary;
use summary::Summary;

mod summoner;
use summoner::Summoner;

use crate::core;
use crate::core::game::Queue;
use crate::screen::search_bar::{self, SearchBar};
use crate::theme;
use crate::widget;
pub use core::summoner::Data;

use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, scrollable, text, themer,
    vertical_space,
};
use iced::{Alignment, Element, Length, Task, Theme};
use iced::{border, padding};

use itertools::Itertools;

pub type Error = String;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueFilter {
    #[default]
    All,
    Specific(Queue),
}

impl QueueFilter {
    pub const ALTERNATIVE: [QueueFilter; 7] = [
        QueueFilter::Specific(Queue::Custom),
        QueueFilter::Specific(Queue::Blind),
        QueueFilter::Specific(Queue::Draft),
        QueueFilter::Specific(Queue::Clash),
        QueueFilter::Specific(Queue::BotIntro),
        QueueFilter::Specific(Queue::BotBeginner),
        QueueFilter::Specific(Queue::BotIntermediate),
    ];
}

impl PartialEq<Queue> for QueueFilter {
    fn eq(&self, other: &Queue) -> bool {
        match self {
            Self::All => true,
            Self::Specific(queue) => queue == other,
        }
    }
}

impl std::fmt::Display for QueueFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueFilter::All => f.write_str("All"),
            QueueFilter::Specific(queue) => write!(f, "{queue}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    FetchedData(Result<Data, String>),
    FetchedGames(Result<core::game::Map, String>),

    Game(usize, game::Message),
    Summoner(summoner::Message),
    SearchBar(search_bar::Message),
    RankedOverview(ranked_overview::Message),

    FetchGames(i64),
    QueueFilterChanged(QueueFilter),
    ThemeChanged(Theme),
}

#[derive(Debug, Clone)]
pub struct Profile {
    puuid: String,
    region: core::Region,
    queue_filter: QueueFilter,

    summary: Summary,
    games: Vec<Game>,
    summoner: Summoner,
    search_bar: SearchBar,
    ranked_overview: RankedOverview,
    theme: Theme,
}

impl Profile {
    pub fn from_profile(assets: &mut crate::Assets, profile: Data) -> Self {
        let puuid = profile.summoner.puuid().to_owned();
        let games = profile
            .games
            .iter()
            .map(|game| Game::from_summoner_game(assets, &puuid, game))
            .collect_vec();
        let summary = Summary::from_games(assets, &games);

        Self {
            region: core::Region::default(),
            queue_filter: QueueFilter::default(),
            summary,
            games,
            search_bar: SearchBar::new(),
            summoner: Summoner::from_profile(&profile),
            ranked_overview: RankedOverview::from_profile(assets, &profile),
            theme: Theme::Moonfly,
            puuid,
        }
    }

    pub fn update(&mut self, message: Message, assets: &mut crate::Assets) -> Task<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::QueueFilterChanged(new_filter) => {
                self.queue_filter = new_filter;
            }
            Message::FetchGames(start_time) => {
                return Task::perform(
                    fetch_games(self.puuid.clone(), self.region, Some(start_time)),
                    Message::FetchedGames,
                );
            }
            Message::FetchedGames(Ok(games)) => {
                self.games.extend(
                    games
                        .iter()
                        .map(|(_, game)| Game::from_summoner_game(assets, &self.puuid, game)),
                );
                self.summary = Summary::from_games(assets, &self.games);
            }
            Message::FetchedData(Ok(profile)) => {
                self.puuid = profile.summoner.puuid().to_owned();
                self.summoner = Summoner::from_profile(&profile);
                self.games = profile
                    .games
                    .iter()
                    .map(|game| Game::from_summoner_game(assets, profile.summoner.puuid(), game))
                    .collect();
                self.summary = Summary::from_games(assets, &self.games);
                self.ranked_overview = RankedOverview::from_profile(assets, &profile);
            }
            Message::FetchedData(Err(error)) => panic!("failed: {error}"),
            Message::FetchedGames(Err(error)) => panic!("failed: {error}"),
            Message::Game(index, message) => {
                if let Some(game::Event::NamePressed(riot_id)) = self
                    .games
                    .get_mut(index)
                    .and_then(|game| game.update(message))
                {
                    return riot_id
                        .name
                        .as_ref()
                        .zip(riot_id.tagline.as_ref())
                        .map(|(name, tag)| {
                            Task::perform(
                                fetch(format!("{name}#{tag}"), self.region),
                                Message::FetchedData,
                            )
                        })
                        .unwrap_or(Task::none());
                }
            }
            Message::Summoner(message) => {
                if let Some(event) = self.summoner.update(message) {
                    match event {
                        summoner::Event::UpdateProfile(name) => {
                            if self.games.is_empty() {
                                return Task::perform(
                                    fetch_games(self.puuid.clone(), self.region, None),
                                    Message::FetchedGames,
                                );
                            }

                            return Task::perform(fetch(name, self.region), Message::FetchedData);
                        }
                    }
                }
            }
            Message::SearchBar(message) => {
                if let Some(event) = self.search_bar.update(message) {
                    match event {
                        search_bar::Event::SearchRequested { riot_id, region } => {
                            self.region = region;

                            return Task::perform(fetch(riot_id, region), Message::FetchedData);
                        }
                    }
                }
            }
            Message::RankedOverview(message) => self.ranked_overview.update(message),
        }

        Task::none()
    }

    pub fn timeline(&self) -> Element<'_, Message> {
        let games = self
            .games
            .iter()
            .enumerate()
            .filter(|(_, game)| self.queue_filter == game.queue())
            .map(|(i, game)| game.view().map(move |message| Message::Game(i, message)))
            .collect_vec();

        if games.is_empty() {
            return container(text("No games found...").size(20))
                .padding(8)
                .center_x(682)
                .into();
        }

        let load_more = button(container("Show more").center_x(Length::Fill))
            .style(theme::show_more)
            .width(Length::Fill)
            .on_press_maybe(
                self.games
                    .last()
                    .map(|g| Message::FetchGames(g.started_at().unix_timestamp())),
            );

        let games = column(games)
            .push(load_more)
            .width(Length::Fill)
            .clip(true)
            .padding(padding::right(12))
            .spacing(4)
            .align_x(Alignment::Center);

        column![
            container(self.summary.view()),
            scrollable(games)
                .style(theme::scrollable)
                .width(Length::Fill)
        ]
        .max_width(680)
        .align_x(Alignment::Center)
        .spacing(4)
        .into()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let pick_list = widget::pick_list(Theme::ALL, Some(&self.theme), theme_picker)
            .on_select(Message::ThemeChanged);
        // .style(|theme, status| theme::queue_picklist(false, theme, status))
        // .menu_style(theme::region_menu),

        let top_bar = container(
            row![
                theme::logo(),
                horizontal_space().width(Length::FillPortion(2)),
                self.search_bar.view().map(Message::SearchBar),
                horizontal_space().width(Length::FillPortion(2)),
                pick_list,
            ]
            .align_y(Alignment::Center),
        )
        .padding(8)
        .style(|theme| container::Style {
            border: border::rounded(0),
            ..theme::dark(theme)
        });

        let content = column![
            self.summoner.view().map(Message::Summoner),
            filter_bar(self.queue_filter),
            row![
                self.ranked_overview.view().map(Message::RankedOverview),
                container(self.timeline())
                    .width(Length::Shrink)
                    .style(theme::timeline),
            ]
            .spacing(8),
        ]
        .width(968)
        .spacing(8)
        .padding(8);

        container(column![
            top_bar,
            vertical_space().height(16),
            container(content).center_x(Length::Fill),
        ])
        .height(Length::Fill)
        .style(theme::timeline)
        .into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn theme_picker<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let colors = column![
        row![
            container(vertical_space().width(8).height(8)).style(container::primary),
            container(vertical_space().width(8).height(8)).style(container::secondary),
        ]
        .spacing(2),
        row![
            container(vertical_space().width(8).height(8)).style(container::success),
            container(vertical_space().width(8).height(8)).style(container::danger),
        ]
        .spacing(2),
    ]
    .padding(2)
    .spacing(2);

    let content = row![
        themer(
            theme.clone(),
            container(colors).style(container::rounded_box)
        ),
        text(theme.to_string())
    ]
    .spacing(4);

    container(content).padding(4).into()
}

fn filter_bar<'a>(selected: QueueFilter) -> Element<'a, Message> {
    let queue_button = |queue: QueueFilter| -> Element<'_, Message> {
        button(text!("{queue}").size(12))
            .style(move |theme, status| theme::queue_filter(theme, status, selected == queue))
            .on_press(Message::QueueFilterChanged(queue))
            .into()
    };

    let picked = Some(selected).filter(|queue| QueueFilter::ALTERNATIVE.contains(queue));

    container(
        row![
            queue_button(QueueFilter::All),
            queue_button(QueueFilter::Specific(Queue::RankedSolo)),
            queue_button(QueueFilter::Specific(Queue::RankedFlex)),
            queue_button(QueueFilter::Specific(Queue::ARAM)),
            pick_list(
                QueueFilter::ALTERNATIVE,
                picked,
                Message::QueueFilterChanged
            )
            .text_size(12)
            .placeholder("Queue type")
            .style(move |theme, status| theme::queue_picklist(picked.is_some(), theme, status))
            .menu_style(theme::region_menu),
        ]
        .spacing(4),
    )
    .width(Length::Fill)
    .padding(8)
    .style(theme::dark)
    .into()
}

pub async fn fetch_games(
    puuid: String,
    region: core::Region,
    start_time: Option<i64>,
) -> Result<game::Map, String> {
    use futures::TryFutureExt;

    let worker_url = dotenv_codegen::dotenv!("WORKER_URL");
    let mut path = format!("{worker_url}/matches/{puuid}");
    if let Some(time) = start_time {
        path.push_str(&format!("?end_time={time}"));
    }

    tracing::info!("Requesting `{puuid}` ({region}) to {path}");

    reqwest::get(path)
        .map_err(|e| e.to_string())
        .await?
        .bytes()
        .map_err(|e| e.to_string())
        .await
        .map(|bytes| game::Map::decode(&bytes))
}

#[cfg(not(feature = "dummy"))]
pub async fn fetch(name: String, region: core::Region) -> Result<core::summoner::Data, String> {
    use futures::TryFutureExt;

    let worker_url = dotenv_codegen::dotenv!("WORKER_URL");
    let path = format!("{worker_url}/summoner/{region}/{}", name.replace("#", "-"));
    tracing::info!("Requesting `{name}` ({region}) to {path}");

    reqwest::get(path)
        .map_err(|e| e.to_string())
        .await?
        .bytes()
        .map_err(|e| e.to_string())
        .await
        .map(|bytes| core::summoner::Data::decode(&bytes))
}

#[cfg(feature = "dummy")]
pub async fn fetch(name: String, _region: core::Region) -> Result<core::summoner::Data, String> {
    let mut id = name.split("#");
    let (name, tag) = (id.next().unwrap_or("someone"), id.next().unwrap_or("foo"));

    Ok(core::summoner::Data::dummy(name, tag))
}
