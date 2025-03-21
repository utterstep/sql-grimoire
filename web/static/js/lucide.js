import {
    createIcons,
    AlertCircle,
    BookOpen,
    Check,
    CheckCircle2,
    ChevronLeft,
    Database,
    Edit,
    Heart,
    Plus,
    Send,
    TableProperties,
    Trash2,
} from 'https://esm.sh/lucide@0.469.0';

const icons = {
    AlertCircle,
    BookOpen,
    Check,
    CheckCircle2,
    ChevronLeft,
    Database,
    Edit,
    Heart,
    Plus,
    Send,
    TableProperties,
    Trash2,
};

document.documentElement.addEventListener('turbo:render', () => {
    createIcons({icons});
});

createIcons({icons});
