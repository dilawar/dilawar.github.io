---
title: Running pydoit tasks in a custom directory 65a701c290744d209e09641eaa4ae9b8
updated: 2026-01-14 22:55:24Z
created: 2026-01-15 04:51:09Z
---

# Running pydoit tasks in a custom directory

Published On: November 11, 2022
Author: dilawar
Status: Published
Type: Note
Tags: devops, lang:python

```python
from doit.action import CmdAction

def task_pwd():
    return {
        'actions': [CmdAction('echo "my task"', cwd='./app')],
        'verbosity': 2,
        }
```

See the discussion 

[Using doit across multiple directories.](https://groups.google.com/g/python-doit/c/ZJUIeE7XIZo/m/wVSabADTKNcJ)