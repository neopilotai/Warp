# Warp Terminal Apps - Mermaid Diagrams

This document contains interactive Mermaid diagrams that visualize the architecture and flows. You can render these diagrams in:
- GitHub (paste into markdown files)
- Mermaid Live Editor (https://mermaid.live)
- VS Code with Mermaid extension
- Any Markdown viewer supporting Mermaid

---

## System Architecture Overview

```mermaid
graph TB
    subgraph App["Your Terminal Application"]
        A["Interactive CLI App"]
    end
    
    subgraph Framework["Terminal Apps Framework"]
        Theme["Theme System"]
        KeySet["KeySet System"]
        Workflow["Workflow Engine"]
    end
    
    subgraph State["Central State: TerminalApp"]
        ST["current_theme"]
        AT["available_themes"]
        SK["current_keyset"]
        AK["available_keysets"]
        SC["custom_config"]
    end
    
    subgraph Services["Runtime Services"]
        CL["Color Lookup"]
        KL["Keybind Lookup"]
        CA["Config Access"]
    end
    
    subgraph UI["Presentation Layer"]
        UI1["UI Rendering"]
        UI2["Business Logic"]
    end
    
    A -->|Uses| Theme
    A -->|Uses| KeySet
    A -->|Uses| Workflow
    
    Theme -->|Updates| ST
    Theme -->|Updates| AT
    KeySet -->|Updates| SK
    KeySet -->|Updates| AK
    
    ST --> CL
    AT --> CL
    SK --> KL
    AK --> KL
    SC --> CA
    
    CL --> UI1
    KL --> UI1
    CA --> UI2
    UI1 --> App
    UI2 --> App
```

---

## Theme Loading Pipeline

```mermaid
graph LR
    A["themes/ directory"] -->|Read| B["ConfigLoader"]
    B -->|Parse YAML| C["Validate Structure"]
    C -->|Validate Colors| D["Create Theme Objects"]
    D -->|Register| E["TerminalApp"]
    E -->|Store in HashMap| F["available_themes"]
    F -->|app.set_theme| G["current_theme"]
    G -->|get_color| H["UI Components"]
    
    C -.->|Invalid| I["ThemeError"]
    D -.->|Error| I
```

---

## Keyset Inheritance & Resolution

```mermaid
graph TB
    subgraph Base["Base Keyset: vim"]
        B1["editor:save → ctrl-s"]
        B2["editor:undo → ctrl-z"]
        B3["editor:quit → ctrl-q"]
    end
    
    subgraph Custom["Custom Keyset: vim_custom"]
        C1["base: vim"]
        C2["editor:quit → :q (override)"]
        C3["editor:new → ctrl-n (new)"]
    end
    
    subgraph Merged["Final Merged Keyset"]
        M1["editor:save → ctrl-s ✓"]
        M2["editor:undo → ctrl-z ✓"]
        M3["editor:quit → :q ✓ (overridden)"]
        M4["editor:new → ctrl-n ✓ (new)"]
    end
    
    Base -->|Inherit| Custom
    Custom -->|Merge| Merged
```

---

## Workflow Execution State Machine

```mermaid
graph TB
    A["Start Workflow"] -->|Load Steps| B["Initialize Context"]
    B -->|Set Variables| C["Step 1: Initial"]
    
    C -->|Has Condition?| D{Condition Eval}
    D -->|False| E["Skip Step"]
    D -->|True| F["Execute Step"]
    
    E -->|Next Step| G["Get Next Steps"]
    F -->|Success| G
    F -->|Failure| H["Execute on_failure Steps"]
    
    G -->|More Steps?| I{Steps Remaining?}
    I -->|Yes| J["Move to Next Step"]
    J -->|Evaluate| D
    I -->|No| K["Workflow Complete"]
    
    H -->|More Steps?| I
    
    E -.->|Skipped| L["Log Skip"]
    F -.->|Executed| M["Log Execution"]
    L -.-> G
    M -.-> G
```

---

## Condition Evaluation Flow

```mermaid
graph LR
    A["Condition Object"] -->|Variable| B["Get from Context"]
    B -->|Found| C["Variable Value"]
    B -->|Not Found| D["Return False"]
    
    C -->|Operator| E{Evaluate}
    
    E -->|equals| F["value == context_value"]
    E -->|not_equals| G["value != context_value"]
    E -->|contains| H["value.contains context_value"]
    E -->|matches| I["glob_match value pattern"]
    
    F -->|Result| J["Boolean Result"]
    G -->|Result| J
    H -->|Result| J
    I -->|Result| J
    
    J -->|true| K["Execute Step"]
    J -->|false| L["Skip Step"]
```

---

## Event Flow: Keybinding to Action

```mermaid
sequenceDiagram
    User->>Terminal: Press Ctrl+S
    Terminal->>App: key_event(ctrl-s)
    App->>Keyset: get_keybinding(keysym)
    Keyset->>HashMap: lookup(ctrl-s)
    HashMap-->>Keyset: editor:save
    Keyset-->>App: editor:save
    App->>Handler: execute_action(editor:save)
    Handler->>File: save_buffer()
    File-->>Handler: success
    Handler-->>User: Saved ✓
```

---

## Event Flow: Theme Change

```mermaid
sequenceDiagram
    User->>App: set_theme(dark)
    App->>HashMap: lookup_theme(dark)
    HashMap-->>App: Some(Theme)
    App->>State: update_current_theme(Theme)
    State-->>App: ✓
    App->>UI: invalidate_color_cache()
    UI->>App: get_color(accent)
    App->>Theme: resolve_color(accent)
    Theme-->>App: #007acc
    App-->>UI: #007acc
    UI->>Screen: re_render_with_color()
```

---

## Concurrent Access Pattern

```mermaid
graph TB
    subgraph T1["Thread 1"]
        T1A["Acquire Lock"]
        T1B["Read get_color"]
        T1C["Release Lock"]
    end
    
    subgraph T2["Thread 2"]
        T2A["Wait for Lock"]
        T2B["Set new theme"]
        T2C["Release Lock"]
    end
    
    subgraph T3["Thread 3"]
        T3A["Wait for Lock"]
        T3B["Get keybinding"]
        T3C["Release Lock"]
    end
    
    T1A -->|0µs| T1B
    T1B -->|0.1µs| T1C
    
    T2A -->|Blocked| T2A
    T2A -->|After T1C| T2B
    T2B -->|1µs| T2C
    
    T3A -->|Blocked| T3A
    T3A -->|After T2C| T3B
    T3B -->|0.1µs| T3C
    
    T1C -.->|Unlock| T2A
    T2C -.->|Unlock| T3A
```

---

## Config Loading & Validation Pipeline

```mermaid
graph TB
    A["Config File"] -->|Read| B["ConfigLoader"]
    B -->|Parse YAML| C{Valid YAML?}
    
    C -->|No| D["YamlError"]
    D -->|Return| E["Err(ConfigError)"]
    
    C -->|Yes| F["Extract Fields"]
    F -->|Validate| G{Complete?}
    
    G -->|No| H["MissingFieldError"]
    H -->|Return| E
    
    G -->|Yes| I{Theme or Keyset?}
    
    I -->|Theme| J["Validate Colors"]
    I -->|Keyset| K["Validate Bindings"]
    
    J -->|Check Hex| L{Valid?}
    K -->|Check Format| M{Valid?}
    
    L -->|No| N["InvalidColor"]
    M -->|No| O["InvalidBinding"]
    N -->|Return| E
    O -->|Return| E
    
    L -->|Yes| P["Create Object"]
    M -->|Yes| P
    P -->|Return| Q["Ok(Object)"]
```

---

## Custom Theme Creation Flow

```mermaid
graph TB
    A["User Creates: themes/neon.yaml"] -->|Syntax| B["Valid YAML?"]
    B -->|No| C["Parse Error"]
    B -->|Yes| D["ConfigLoader Discovers"]
    
    D -->|Loads| E["parse_yaml"]
    E -->|Validates| F["Has Required Fields?"]
    F -->|No| G["Validation Error"]
    F -->|Yes| H["Create Theme"]
    
    H -->|Register| I["app.register_themes"]
    I -->|Store| J["available_themes"]
    J -->|User Selects| K["app.set_theme(neon)"]
    K -->|Activate| L["current_theme"]
    L -->|Use in App| M["get_color() uses neon colors"]
```

---

## Error Handling Decision Tree

```mermaid
graph TD
    A["Operation"] -->|Try| B{Error?}
    
    B -->|No| C["Continue"]
    B -->|Yes| D{ErrorType?}
    
    D -->|IoError| E["File not found/readable"]
    E -->|Action| F["Fallback to default theme"]
    
    D -->|YamlError| G["Invalid YAML syntax"]
    G -->|Action| H["Log error, skip file"]
    
    D -->|ValidationError| I["Invalid field values"]
    I -->|Action| J["Log error, use defaults"]
    
    D -->|NotRegistered| K["Theme/Keyset not found"]
    K -->|Action| L["Return error, don't change"]
    
    F --> M["Graceful Degradation ✓"]
    H --> M
    J --> M
    L --> M
```

---

## Data Structure Dependencies

```mermaid
graph TB
    A["TerminalApp"] -->|owns| B["TerminalAppState"]
    B -->|owns| C["current_theme"]
    B -->|owns| D["available_themes"]
    B -->|owns| E["current_keyset"]
    B -->|owns| F["available_keysets"]
    B -->|owns| G["custom_config"]
    
    C -->|references| H["Theme"]
    D -->|HashMap| H
    
    E -->|references| I["KeySet"]
    F -->|HashMap| I
    
    H -->|contains| J["TerminalColors"]
    H -->|contains| K["custom_colors"]
    
    J -->|contains| L["normal ColorPalette"]
    J -->|contains| M["bright ColorPalette"]
    
    L -->|Vec| N["8 Colors"]
    M -->|Vec| N
    
    I -->|contains| O["bindings HashMap"]
    O -->|Maps| P["action → keysym"]
```

---

## Workflow Variable Resolution

```mermaid
graph LR
    A["Workflow Definition"] -->|variables:| B["Environment: production"]
    B -->|Store in| C["ExecutionContext"]
    
    D["Step 1"] -->|Uses| C
    E["Step 2 Condition"] -->|Variable: environment| F["Lookup in Context"]
    F -->|Found| G["Value: production"]
    G -->|Compare| H["operator: equals"]
    H -->|Value: production| I{Match?}
    I -->|Yes| J["Execute Step"]
    I -->|No| K["Skip Step"]
```

---

## Performance Characteristics

```mermaid
graph TB
    A["Operation Complexity"] -->|O1| B["Create Theme: &lt;1µs"]
    A -->|O1| C["Create KeySet: &lt;1µs"]
    A -->|O1| D["Get Color: &lt;1µs"]
    A -->|O1| E["Get Keybinding: &lt;1µs"]
    A -->|On| F["Load Directory: 10-100ms"]
    A -->|On| G["Load Single YAML: 1-10ms"]
    
    B -->|HashMap| H["O1 Constant Time"]
    C -->|HashMap| H
    D -->|HashMap| H
    E -->|HashMap| H
    F -->|File I/O| I["Linear in file count"]
    G -->|File I/O| J["Linear in file size"]
    
    H -->|Result| K["Microsecond Performance"]
    I -->|Result| L["Millisecond Performance"]
    J -->|Result| L
```

---

## Accessibility Feature Tree

```mermaid
graph TB
    A["Accessibility Features"]
    
    A -->|Color| B["Contrast Checking"]
    B -->|WCAG AA| C["4.5:1 Text Ratio"]
    B -->|Fallbacks| D["High Contrast Theme"]
    
    A -->|Keyboard| E["Full Keybinding Support"]
    E -->|Multiple Paradigms| F["Vim, Emacs, VSCode"]
    E -->|Customizable| G["Override Keys"]
    
    A -->|Screen Reader| H["Description Support"]
    H -->|Workflow| I["Steps Logged to Stdout"]
    H -->|Help| J["Keybinding List Available"]
    
    A -->|Motor| K["Customizable Keysets"]
    K -->|Reduce Strain| L["Reassign Keys as Needed"]
    K -->|Fast Repeat| M["Configurable in Metadata"]
```

---

## Localization Architecture

```mermaid
graph TB
    A["Application"]
    
    A -->|Messages| B["locale/messages_en.yaml"]
    A -->|Messages| C["locale/messages_es.yaml"]
    A -->|Messages| D["locale/messages_fr.yaml"]
    
    A -->|Themes| E["themes/dark_en.yaml"]
    A -->|Themes| F["themes/dark_es.yaml"]
    A -->|Themes| G["themes/dark_fr.yaml"]
    
    B -->|get_localized_string| H["Message Key"]
    C -->|get_localized_string| H
    D -->|get_localized_string| H
    
    H -->|Resolved| I["Localized Output"]
    
    E -->|Name/Description| J["Localized Metadata"]
    F -->|Name/Description| J
    G -->|Name/Description| J
```

---

## Running These Diagrams

### **Option 1: Mermaid Live Editor**
1. Visit https://mermaid.live
2. Copy a diagram from above
3. Paste into the editor
4. View the rendered diagram

### **Option 2: GitHub Markdown**
1. Create a `.md` file in your repo
2. Paste diagrams in `\`\`\`mermaid` blocks
3. GitHub auto-renders them

### **Option 3: VS Code**
1. Install "Markdown Preview Mermaid Support" extension
2. View any `.md` file with diagrams
3. Preview renders automatically

### **Option 4: Local Node.js**
```bash
npm install @mermaid-js/mermaid-cli
mmdc -i MERMAID_DIAGRAMS.md -o architecture.svg
```

---

## Additional Resources

- **ARCHITECTURE.md**: Detailed ASCII diagrams and explanations
- **MODULE_DOCUMENTATION.md**: API reference for each module
- **QUICK_REFERENCE.md**: Quick lookup guide for developers
- **TERMINAL_APPS_SUMMARY.md**: Complete system overview
