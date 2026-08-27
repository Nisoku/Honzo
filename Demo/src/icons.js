import { createElement } from "lucide";
import {
  Bookmark,
  BookUp,
  ChevronLeft,
  ChevronRight,
  Code2,
  Contrast,
  Download,
  File,
  FilePlus,
  FileText,
  FileUp,
  Grid3x3,
  GripVertical,
  Image,
  Info,
  Menu,
  MessageSquare,
  Moon,
  Plus,
  RefreshCw,
  RotateCcw,
  Save,
  Search,
  Settings,
  Shield,
  Sigma,
  Sun,
  Trash2,
  Type,
  Wrench,
  X,
} from "lucide";

const map = {
  Bookmark,
  BookUp,
  ChevronLeft,
  ChevronRight,
  Code2,
  Contrast,
  Download,
  File,
  FilePlus,
  FileText,
  FileUp,
  Grid3x3,
  GripVertical,
  Image,
  Info,
  Menu,
  MessageSquare,
  Moon,
  Plus,
  RefreshCw,
  RotateCcw,
  Save,
  Search,
  Settings,
  Shield,
  Sigma,
  Sun,
  Trash2,
  Type,
  Wrench,
  X,
};

export const icons = map;

const cache = new Map();

export function icon(name, size = 20, strokeWidth = 2, attrs = {}) {
  const key = `${name}-${size}-${strokeWidth}-${JSON.stringify(attrs)}`;
  if (cache.has(key)) return cache.get(key);
  const iconData = map[name];
  if (!iconData) return "";
  const el = createElement(iconData, {
    width: size,
    height: size,
    strokeWidth,
    ...attrs,
  });
  const str = el.outerHTML;
  cache.set(key, str);
  return str;
}
