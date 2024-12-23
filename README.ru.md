
# О проекте

Утилита расширяет стандартный функционал оконных менеджеров i3 и Sway.

Среди основных возможностей:

-   Workspace groups. Полезно, когда вы работаете сразу над множеством проектов, и хочется функционал стандартных рабочих
    пространств ($mod+1, $mod+2, … $mod+9) расширить до нескольких групп, по проектам. Можно настроить так, чтобы
    кнопки $mod+N переключали рабочие пространства только в текущей группе, и одновременно добавить возможность создания
    дополнительных групп и переключения между ними.
-   Обработчик событий. Есть простая возможность через конфигурационный файл задать логику обработки событий.


# Установка

В данный установка возможна только из исходников.

Установите Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Установите проект:

```bash
cargo install i3im
```

# С чего начать

Утилита имеет развитую командую строку. Не бойтесь позвать ее с параметром help.

Для начала, надо создать конфиг. Это можно сделать командой:

```bash
i3im config generate > $HOME/.config/i3im.yaml
```

# Группы рабочих пространств

Пропишите в конфиг i3:

```ini
bindsym $mod+1 exec i3im focus workspace 1
bindsym $mod+2 exec i3im focus workspace 2
bindsym $mod+3 exec i3im focus workspace 3
bindsym $mod+4 exec i3im focus workspace 4
bindsym $mod+5 exec i3im focus workspace 5
bindsym $mod+6 exec i3im focus workspace 6
bindsym $mod+7 exec i3im focus workspace 7
bindsym $mod+8 exec i3im focus workspace 8
bindsym $mod+9 exec i3im focus workspace 9
bindsym $mod+0 exec i3im focus workspace 10

bindsym $mod+Shift+1 exec i3im move window-to-group-workspace 1
bindsym $mod+Shift+2 exec i3im move window-to-group-workspace 2
bindsym $mod+Shift+3 exec i3im move window-to-group-workspace 3
bindsym $mod+Shift+4 exec i3im move window-to-group-workspace 4
bindsym $mod+Shift+5 exec i3im move window-to-group-workspace 5
bindsym $mod+Shift+6 exec i3im move window-to-group-workspace 6
bindsym $mod+Shift+7 exec i3im move window-to-group-workspace 7
bindsym $mod+Shift+8 exec i3im move window-to-group-workspace 8
bindsym $mod+Shift+9 exec i3im move window-to-group-workspace 9
bindsym $mod+Shift+0 exec i3im move window-to-group-workspace 10

bindsym $mod+Shift+s exec i3im focus wm-workspace "`i3im list wm-workspaces | rofi -dmenu -p 'Switch to workspace'`"
```

Обратите внимание, что в последней команде используется rofi.

Теперь с помощью кнопки $mod+Shift+s можно создать новую группу рабочих простанств. Внутри группы переключение между
пространствами происходит привычными кнопками. Переключиться обратно в группу по умолчанию (она не имеет названия) можно
с помощью того же сочетания клавиш — $mod+Shift+s. Таким же способом при необходимости можно создать несколько рабочих
групп.


# Обработчик событий

i3im умеет работать в режиме обработчика событий. Для этого надо запустить:

```bash
i3im listen
```

По умолчанию, список обработчиков пуст. Их необходимо описать в конфиге, в секциях `window_event_handlers` и
`workspace_event_handlers`. Каждый элемент этих массивов состоит из двух элементов:

- `condition_list`: список условий, которые должны выполниться, чтобы данный обработчик сработал.
- `action`: действие, которое необходимо выполнить при наступлении условий.

Полный список доступных вариантов условий и действий можно посмотреть вызвав справку по конфигу:

```bash
i3im config documentation
```

## Пример: увеличение яркости для полноэкранных окон

В конфиг дописать:

```yaml
window_event_handlers:
  # Обработчик события на переход окна в полноэкранный режим
  - condition_list:
      # Тип события: изменение полноэкранного режима
      - EventType: [ FullscreenMode ]
      # Новый статус полноэкранного режима: активирован
      - NodeFullscreenMode: Fullscreen
    action:
      # вызвать шелл-команду
      ShellCommand:
        command: ~/.config/i3/scripts/brightness_autoset.sh 100

  # Обработчик события на выход окна из полноэкранного режима
  - condition_list:
      # Тип события: изменение полноэкранного режима
      - EventType: [ FullscreenMode ]
      # Новый статус полноэкранного режима: не активен
      - NodeFullscreenMode: None
    action:
      # вызвать шелл-команду
      ShellCommand:
        command: ~/.config/i3/scripts/brightness_autoset.sh
```

Содержимое скрипта `~/.config/i3/scripts/brightness_autoset.sh`:

```bash
#! /bin/sh

DEFAULT_BRIGHTNESS='20'
STATE_FILE="$XDG_STATE_HOME/brightness_autoset"

BRIGHTNESS="$1"
if [ -z "$BRIGHTNESS" ]; then
    BRIGHTNESS="$(cat "$STATE_FILE" 2>/dev/null || echo "$DEFAULT_BRIGHTNESS")"
    brightnessctl set "${BRIGHTNESS}%"
    exit 0
fi

brightnessctl brightnessctl -m | cut -d, -f4 | tr -d % > "$STATE_FILE"

brightnessctl set "${BRIGHTNESS}%"
```

## Пример: автоматическое закрытие окна Jitsi о включенном шаринге

Многих раздражает, что система видеоконференций Jitsi автоматом показывает окошко с уведомлением о шаринге. Его можно
автоматически закрывать таким хаком:

```yaml
window_event_handlers:
  - condition_list:
      # Тип события: создание нового окна
      - EventType: [ New ]
      # Заголовок окна должен совпадать с регулярным выражением
      - Title:
          Regex:
            regex: 'jitsi.*is sharing'
    action:
      # вызвать шелл-команду
      ShellCommand:
        command: xdotool windowclose "$I3IM_CONTAINER_WINDOW_ID"
```

## Пример: получение переменных окружения

В примере выше вы могли обратить внимание на переменную окружения `$I3IM_CONTAINER_WINDOW_ID`. Получить полный список
переменных можно навешав пустое условие на событие:

```yaml
window_event_handlers:
  - condition_list: []
    action:
      # вызвать шелл-команду
      ShellCommand:
        command: env | grep I3IM_ > /tmp/i3im-environment-list.txt
```

Аналогичным образом можно сделать и для событий рабочих пространств.

# Отладка

По умолчанию, утилита пишет логи в syslog. Можно заставить писать лог в консоль, запустив с переменной окружения:

```bash
RUST_LOG=debug i3im listen

# то же самое, с уровнем логирования "info":
RUST_LOG=info i3im listen
```
