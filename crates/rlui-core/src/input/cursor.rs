use num_traits::Signed;

use crate::input::{IntPos, Pos};

pub trait Cursor<I: IntPos> {
    /// Returns the current position of the cursor.
    fn pos(&self) -> Pos<I>;

    /// Go directly to a specific row.
    fn goto_row(&mut self, row: I);

    /// Go directly to a specific column.
    fn goto_column(&mut self, column: I);

    /// Go directly to a specific position.
    fn goto(&mut self, pos: Pos<I>) {
        self.goto_row(pos.row);
        self.goto_column(pos.col);
    }

    fn move_by(&mut self, row_delta: I::Signed, column_delta: I::Signed) {
        self.move_by_rows(row_delta);
        self.move_by_columns(column_delta);
    }

    /// Move by a certain number of rows.
    fn move_by_rows(&mut self, row_delta: I::Signed) {
        let mut row = self.pos().row;

        if row_delta.is_positive() {
            row = row.saturating_add(I::abs(row_delta));
        } else {
            row = row.saturating_sub(I::abs(row_delta));
        }

        self.goto_row(row);
    }

    /// Move by a certain number of columns.
    fn move_by_columns(&mut self, column_delta: I::Signed) {
        let mut column = self.pos().col;

        if column_delta.is_positive() {
            column = column.saturating_add(I::abs(column_delta));
        } else {
            column = column.saturating_sub(I::abs(column_delta));
        }

        self.goto_column(column);
    }
}
