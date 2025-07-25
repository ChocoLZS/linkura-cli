# linkura-cli

## 免责声明

1. **用途说明**】

    本仓库所有代码、文档及资源均为**个人学习与研究目的**而创建，仅代表作者个人实践，**不构成任何正式建议或承诺**。

2. **无担保责任**

    - 作者不对代码的准确性、完整性或适用性作任何担保。

    - 使用者应自行承担因使用、修改或分发本仓库内容而产生的所有风险。

3. **法律与合规性**

    - 使用者需确保其用途符合所在地法律法规及开源许可要求。

    - 若内容涉及第三方权益（如知识产权、隐私权等），请立即联系作者删除。

4. **免责范围**

    - 因使用本仓库内容导致的直接或间接损失（如数据丢失、系统故障、法律纠纷等）。

    - 他人基于本仓库内容进行的二次开发或商业行为产生的后果。

## Workflow

```mermaid
flowchart TD
    A[linkura-cli] --> B{Check config file exist}
    B -->|Y| C[Prepare]
    B -->|N| E[Prompt for username]
    E --> F[Prompt for password]
    F --> G[Login]
    G --> H[Save token]
    H --> C
    C --> I{Run command}
    I --> N[Return result]
```
