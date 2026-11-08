use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListDirection, Paragraph, StatefulWidget, Widget},
};

use crate::app::App;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Min(0)])
            .split(area);

        let explorer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        // Folders
        let folder_list_display = List::new(self.folders.iter().map(String::as_str))
            .block(
                Block::bordered()
                    .title("Folders")
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan).bold())
            .highlight_symbol("  ")
            .direction(ListDirection::BottomToTop);

        StatefulWidget::render(
            folder_list_display,
            explorer_chunks[0],
            buf,
            &mut self.folder_list,
        );

        // Files
        let file_list_display = List::new(self.files.iter().map(String::as_str))
            .block(
                Block::bordered()
                    .title("Files")
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan).bold())
            .highlight_symbol("  ")
            .direction(ListDirection::BottomToTop);

        StatefulWidget::render(
            file_list_display,
            explorer_chunks[1],
            buf,
            &mut self.file_list,
        );

        let text = format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            self.counter
        );

        let paragraph = Paragraph::new(text)
            .block(
                Block::bordered()
                    .title("poke")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(chunks[1], buf);
    }
}
