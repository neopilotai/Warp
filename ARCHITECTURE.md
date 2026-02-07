# Warp Terminal Apps - Architecture & Diagrams

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Your Terminal Application                   │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                    Uses Framework APIs
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
        ▼                      ▼                      ▼
   ┌─────────┐           ┌─────────┐          ┌─────────────┐
   │  Theme  │           │ KeySet  │          │  Workflow   │
   │ System  │           │ System  │          │   Engine    │
   └────┬────┘           └────┬────┘          └────┬────────┘
        │                     │                    │
        │  Updates            │  Updates           │ Updates
        ▼                     ▼                    ▼
   ┌─────────────────────────────────────────────────────────┐
   │              TerminalApp (Central State)               │
   │  ┌─────────────────────────────────────────────────────┐
   │  │ • current_theme                                     │
   │  │ • available_themes (HashMap)                        │
   │  │ • current_keyset                                    │
   │  │ • available_keysets (HashMap)                       │
   │  │ • custom_config (HashMap)                           │
   │  └─────────────────────────────────────────────────────┘
   └───────────────────────┬─────────────────────────────────┘
                           │
          Provides State & Configuration
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
    ┌────────┐        ┌────────┐        ┌──────────┐
    │ Color  │        │ Keybind│        │ Config   │
    │ Lookup │        │ Lookup │        │ Access   │
    └────────┘        └────────┘        └──────────┘
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
        ┌──────────────────┴──────────────────┐
        │                                     │
        ▼                                     ▼
   ┌──────────────┐                   ┌──────────────┐
   │  UI Rendering│                   │   Business   │
   │   Components │                   │    Logic     │
   └──────────────┘                   └──────────────┘
```

## Module Interaction Flow

```
Application Initialization
│
├─ Create TerminalApp
│  │
│  ├─ Load Themes
│  │  └─ ConfigLoader::load_themes_from_directory()
│  │     └─ Parse YAML → Theme objects
│  │
│  ├─ Load Keysets
│  │  └─ ConfigLoader::load_keysets_from_directory()
│  │     └─ Parse YAML → KeySet objects
│  │
│  └─ Register Both
│     ├─ app.register_themes(themes)
│     └─ app.register_keysets(keysets)
│
├─ Set Active Configuration
│  ├─ app.set_theme("dark")
│  ├─ app.set_keyset("vim")
│  └─ app.set_config("key", "value")
│
└─ Application Ready
   ├─ app.get_color("accent") → Use in rendering
   ├─ app.get_keybinding("save") → Use in input handling
   ├─ app.get_config("debug") → Use in logic
   └─ Use workflows for task execution
```

## Theme Resolution Flow

```
User requests color "accent"
        │
        ▼
app.get_color("accent")
        │
        ▼
app.current_theme.as_ref()?.get_color("accent")
        │
        ├──────────────────┬──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
    "background"      "foreground"       "accent"
    built-in?         built-in?          built-in?
        │                  │                  │
        ├──────────────────┴──────────────────┘
        │                  │
        │                  ▼
        │         Check custom_colors HashMap
        │                  │
        └──────────────────┴─────────────────┐
                           │                 │
                    Found ──┘                 └── Not found
                           │                      │
                           ▼                      ▼
                      Return Some(color)    Return None
```

## Keybinding Resolution Flow

```
User presses key combination "ctrl-s"
        │
        ▼
Handle input event
        │
        ▼
app.get_keybinding("editor:save")
        │
        ▼
app.current_keyset.as_ref()?.get_binding("editor:save")
        │
        ▼
Check keyset.bindings HashMap
        │
        ├─ Found ────────────┐
        │                    ▼
        │             Execute action
        │
        └─ Not found ────────┐
                            ▼
                      Do nothing / Error
```

## Workflow Execution Flow

```
User triggers workflow "deploy"
        │
        ▼
Load ExtendedWorkflow "deploy"
        │
        ├─ Set variables: env="production"
        │
        └─ Initialize ExecutionContext
           │
           ▼
        Step 1: "install"
           │
           ├─ Condition? None
           ├─ Execute workflow: "npm:install"
           │
           ├─ Success?
           │  ├─ Yes → Execute next steps ["build"]
           │  └─ No → Execute failure steps ["notify"]
           │
           ▼
        Step 2: "build"
           │
           ├─ Condition:
           │  ├─ Variable: "env"
           │  ├─ Operator: "equals"
           │  └─ Value: "production"
           │
           ├─ Evaluate condition
           │  ├─ Context has: env="production"
           │  ├─ "production" == "production"?
           │  └─ TRUE → Execute step
           │
           ├─ Execute workflow: "npm:build"
           │
           └─ Continue based on result...

        Additional Steps...
           │
           ▼
        Workflow Complete
```

## Condition Evaluation Logic

```
Condition:
├─ variable: "env"
├─ operator: "equals"
└─ value: Some("production")

Context variables:
├─ env = "production"
├─ version = "1.0.0"
└─ timeout = "300"

Evaluation:
        │
        ├─ Get value of "env" from context
        │  └─ Returns Some("production")
        │
        ├─ Apply operator "equals"
        │  └─ "production" == "production"?
        │
        └─ Return true / false
```

## Configuration File Parsing

```
YAML File (theme.yaml)
        │
        ▼
ConfigLoader::load_theme()
        │
        ├─ Read file into string
        │
        ├─ Parse YAML → HashMap
        │
        ├─ Validate structure
        │
        ├─ Create Theme struct
        │  ├─ name: String
        │  ├─ background: String
        │  ├─ foreground: String
        │  ├─ accent: String
        │  ├─ terminal_colors: TerminalColors
        │  │  ├─ normal: ColorPalette
        │  │  └─ bright: ColorPalette
        │  └─ custom_colors: HashMap
        │
        └─ Return Result<Theme, ThemeError>
```

## Error Handling Paths

```
Operation: ConfigLoader::load_theme("path")
        │
        ├─ File exists?
        │  └─ No → IoError → ThemeError::IoError
        │
        ├─ YAML valid?
        │  └─ No → YamlError → ThemeError::YamlError
        │
        ├─ All fields present?
        │  └─ No → YamlError → ThemeError::YamlError
        │
        ├─ Theme registered in app?
        │  └─ No → Can't set → set_theme() returns false
        │
        └─ Success → Ok(Theme)
```

## State Machine: Theme Switching

```
                ┌─────────────────┐
                │   No Theme Set  │
                └────────┬────────┘
                         │
                    set_theme("dark")
                         │
                         ▼
            ┌────────────────────────┐
            │   Dark Theme Active    │
            │ (get_color() works)    │
            └────────┬───────────────┘
                     │
          ┌──────────┴───────────┐
          │                      │
    set_theme("light")    set_theme("custom")
          │                      │
          ▼                      ▼
    ┌──────────────┐      ┌──────────────┐
    │  Light Theme │      │Custom Theme  │
    │  Active      │      │Active        │
    └──────────────┘      └──────────────┘
```

## Keyset Inheritance Model

```
BaseKeyset (vim)
├─ editor:save → "ctrl-s"
├─ editor:undo → "ctrl-z"
├─ editor:quit → "ctrl-q"
└─ editor:find → "/"

        │
        │ inherit from
        ▼

CustomKeyset (vim_custom)
├─ base: "vim"
├─ editor:save → "ctrl-s"    (inherited)
├─ editor:undo → "ctrl-z"    (inherited)
├─ editor:quit → ":q"        (overridden)
├─ editor:find → "?"         (overridden)
└─ editor:new → "ctrl-n"     (new binding)

        │
        │ merged in app
        ▼

FinalKeyset
├─ editor:save → "ctrl-s"
├─ editor:undo → "ctrl-z"
├─ editor:quit → ":q"        (✓ overridden)
├─ editor:find → "?"         (✓ overridden)
└─ editor:new → "ctrl-n"     (✓ new)
```

## Data Structure Relationships

```
TerminalApp
    │
    ├─ current_theme: Theme?
    │   └─ terminal_colors: TerminalColors
    │       ├─ normal: ColorPalette (8 colors)
    │       └─ bright: ColorPalette (8 colors)
    │
    ├─ available_themes: HashMap<String, Theme>
    │   └─ [name] → Theme { ... }
    │
    ├─ current_keyset: KeySet?
    │   └─ bindings: HashMap<String, String>
    │       └─ [action] → key_combination
    │
    ├─ available_keysets: HashMap<String, KeySet>
    │   └─ [name] → KeySet { ... }
    │
    └─ custom_config: HashMap<String, String>
        └─ [key] → value

ExtendedWorkflow
    │
    ├─ name: String
    ├─ steps: Vec<WorkflowStep>
    │   └─ [i] WorkflowStep
    │       ├─ name: String
    │       ├─ workflow: String
    │       ├─ condition: Condition?
    │       │   ├─ variable: String
    │       │   ├─ operator: String
    │       │   └─ value: String?
    │       ├─ on_success: Vec<String>? (step names)
    │       └─ on_failure: Vec<String>? (step names)
    │
    └─ variables: HashMap<String, String>
        └─ [name] → value

ExecutionContext
    └─ variables: HashMap<String, String>
        └─ [name] → value (runtime evaluation)
```

## Performance Profile

```
Operation                  Complexity    Time
─────────────────────────────────────────────
Create Theme               O(1)          < 1µs
Create KeySet              O(1)          < 1µs
Register Theme             O(1)          < 1µs
Set Theme                  O(1)          < 1µs
Get Color                  O(1)          < 1µs
Get Keybinding             O(1)          < 1µs
Load Theme YAML            O(n)          1-10ms (n = file size)
Load Directory             O(m)          10-100ms (m = file count)
Evaluate Condition         O(1)          < 1µs
Add Workflow Step          O(1)          < 1µs
```

## Typical Memory Usage

```
TerminalApp with 1000 themes:
├─ App structure: ~1KB
├─ 1000 Theme objects: ~5MB
│  └─ Average 5KB per theme
├─ 20 KeySet objects: ~100KB
│  └─ Average 5KB per keyset
└─ Config HashMap: ~10KB
  
Total: ~5.1MB (for full Warp theme library)

Typical usage (5-10 themes):
├─ App structure: ~1KB
├─ 5 Theme objects: ~25KB
├─ 5 KeySet objects: ~25KB
└─ Config HashMap: ~10KB

Total: ~61KB
```

## Security Considerations

```
Input Validation
├─ Theme loading: YAML parsing validates structure
├─ Keyset loading: YAML parsing validates structure
├─ Workflow: YAML parsing validates structure
└─ Variables: Stored as strings (type-safe)

No Security Concerns:
├─ No network operations
├─ No command execution
├─ No unsafe code
├─ No dynamic code loading
└─ Input treated as configuration only
```

---

**Diagram Legend**:
- `→` : Data flow or function call
- `├─` : Component/property
- `└─` : Last component/property
- `○` : Optional element
- `?` : Can be None/None

