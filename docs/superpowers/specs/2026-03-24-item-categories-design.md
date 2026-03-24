# Design Doc: Item Categories and Grouped List View

Implement a category system for items in the search list, allowing for future expansion to different item types (e.g., bookmarks, files) with distinct detail views.

## Goals
- Abstract application items into a generic `Item` enum.
- Introduce a `Category` enum to group items.
- Implement grouped sorting (Category -> Name).
- Display category headers in the list view (Integrated Headers).
- Enable category-specific detail views on the right side.

## Architecture

### Data Models (`src/app_info.rs`)

#### `Category` Enum
Defines the types of items supported by the system.
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Applications,
    WebBookmarks,
    Files,
}

impl Category {
    pub fn display_name(&self) -> &str {
        match self {
            Self::Applications => "Applications",
            Self::WebBookmarks => "Web Bookmarks",
            Self::Files => "Files",
        }
    }
}
```

#### `Item` Enum
The unified type for all searchable items.
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Application(AppInfo),
    // Future additions:
    // WebBookmark(BookmarkInfo),
    // File(FileInfo),
}

impl Item {
    pub fn category(&self) -> Category {
        match self {
            Self::Application(_) => Category::Applications,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Application(app) => &app.name,
        }
    }

    pub fn icon_name(&self) -> Option<&str> {
        match self {
            Self::Application(app) => app.icon_name.as_deref(),
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Application(app) => app.description.as_deref(),
        }
    }

    pub fn exec(&self) -> Option<&str> {
        match self {
            Self::Application(app) => app.exec.as_deref(),
        }
    }
}
```

### Logic Changes

#### Sorting
Items will be sorted primarily by their category (using `Category`'s `Ord` implementation) and secondarily by their name.
```rust
items.sort_by(|a, b| {
    match a.category().cmp(&b.category()) {
        std::cmp::Ordering::Equal => a.name().cmp(b.name()),
        other => other,
    }
});
```

### UI Implementation (`src/main.rs`)

#### List Headers (`Integrated Headers`)
Use `gtk4::ListBox::set_header_func` to inject category labels.
- A header is created only when the category of the current row differs from the previous row.
- The header will be a `gtk4::Box` containing a `gtk4::Label` and a `gtk4::Separator` for a clean, integrated look.

#### AppRow Component
Update `AppRow` to accept an `Item` instead of just `AppInfo`.

#### Detail View
The right-side detail view in `App` component's `view!` macro will use a `match` statement on the selected item to render category-specific layouts.
Initially, only the `Application` layout will be implemented, but the structure will support adding others easily.

## Design Details

### CSS
New CSS classes for category headers:
- `.category-header`: Styling for the container.
- `.category-label`: Small, bold, uppercase text for the category name.

## Testing Plan
- Verify that applications are still correctly listed and searchable.
- Confirm that the list is sorted by category (currently only "Applications") and then by name.
- Verify that the category header "Applications" appears at the top of the list.
- Ensure that selecting an item still shows its details and clicking it launches the app.
- (Self-Correction): Since only one category exists now, manually add a dummy category/item temporarily during testing to verify header insertion logic if possible, or trust the `set_header_func` logic with the single category for now.
