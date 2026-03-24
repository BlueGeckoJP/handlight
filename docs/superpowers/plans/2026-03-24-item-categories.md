# Item Categories and Grouped List View Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a category-based grouping and sorting system for items in the search list, with support for distinct detail views.

**Architecture:** Use Rust enums (`Category`, `Item`) for type-safe item management and GTK's `set_header_func` for efficient category header rendering in the list view. Relm4's `view!` macro with `match` will handle dynamic detail views.

**Tech Stack:** Rust, GTK4, Relm4

---

### Task 1: Define Category and Item Enums

**Files:**
- Modify: `src/app_info.rs`

- [ ] **Step 1: Define `Category` enum**
Add the `Category` enum with `Applications`, `WebBookmarks`, and `Files` variants, including a `display_name` method.

- [ ] **Step 2: Define `Item` enum**
Add the `Item` enum wrapping `AppInfo` in an `Application` variant. Implement helper methods: `category()`, `name()`, `icon_name()`, `description()`, and `exec()`.

- [ ] **Step 3: Update `get_installed_apps` signature and logic**
Change return type to `Vec<Item>` and wrap `AppInfo` into `Item::Application`.

- [ ] **Step 4: Update sorting logic**
Implement primary sort by `Category` and secondary sort by name.

- [ ] **Step 5: Verify compilation**
Run: `cargo check`
Expected: Success (ignoring `main.rs` errors for now).

- [ ] **Step 6: Commit**
```bash
git add src/app_info.rs
git commit -m "feat: define Category and Item enums and update data fetching"
```

### Task 2: Refactor AppRow for Generic Items

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Update `AppRow` struct and `FactoryComponent` implementation**
Change `AppRow` to hold an `Item` instead of `AppInfo`. Update `view!` macro to use `self.app.icon_name()` and `self.app.name()`.

- [ ] **Step 2: Commit**
```bash
git add src/main.rs
git commit -m "refactor: update AppRow to use generic Item enum"
```

### Task 3: Update App Component and Detail View

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Update `App` struct fields**
Change `all_apps` and `filtered_apps_data` to `Vec<Item>`.

- [ ] **Step 2: Update `SimpleComponent::init`**
Update data initialization to handle `Item` types.

- [ ] **Step 3: Update `SimpleComponent::update`**
Update `SearchChanged`, `AppClicked`, and other message handlers to work with `Item` methods.

- [ ] **Step 4: Update detail view in `view!` macro**
Refactor the right-side detail view to use a `match` statement on the selected item.

- [ ] **Step 5: Commit**
```bash
git add src/main.rs
git commit -m "feat: update App component and dynamic detail view"
```

### Task 4: Implement Category Headers

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Add CSS for category headers**
Add `.category-header` and `.category-label` styles to `relm4::set_global_css`.

- [ ] **Step 2: Implement `set_header_func` in `SimpleComponent::init`**
Add the header function logic to the `ListBox` within the `init` method.

- [ ] **Step 3: Verify and Test**
Run: `cargo run`
Expected: App launches, shows "Applications" header at the top, items are sorted correctly, and detail view works.

- [ ] **Step 4: Commit**
```bash
git add src/main.rs
git commit -m "feat: implement category headers in list view"
```
