import {
    createIcons,
    BookOpen,
    CheckCircle2,
    Plus,
    Edit,
    Trash2,
    ChevronLeft,
    Database,
    Check,
    Send,
} from 'https://esm.sh/lucide@0.469.0';

const icons = {
    BookOpen,
    CheckCircle2,
    Plus,
    Edit,
    Trash2,
    ChevronLeft,
    Database,
    Check,
    Send,
};

document.documentElement.addEventListener('turbo:render', () => {
    createIcons({icons});
});

createIcons({icons});
